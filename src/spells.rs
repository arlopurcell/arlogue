use std::collections::HashMap;
use std::ops::DerefMut;

use crate::level::Level;
use crate::utils::{AbsoluteLocation, RelativeLocation, Direction};
use crate::monster::Monster;
use constants::MONSTER_SPELLBOOK;

lalrpop_mod!(pub cmdlist); // synthesized by LALRPOP

pub struct Caster {
    pub location: AbsoluteLocation,
    pub energy: u32,
    max_energy: u32,
    energy_regen: u32,
}

impl Caster {
    pub fn simple(location: AbsoluteLocation, energy: u32) -> Caster {
        Caster {
            location: location,
            energy: energy,
            max_energy: energy,
            energy_regen: energy,
        }
    }

    pub fn cast(&mut self, cost: u32) -> bool { // false if not enough energy
        if cost > self.energy { false } 
        else {
            self.energy -= cost;
            true
        }
    }

    pub fn move_to(&mut self, location: AbsoluteLocation) { self.location = location }

    pub fn regen(&mut self) {
        self.energy += self.energy_regen;
        if self.energy > self.max_energy {
            self.energy = self.max_energy;
        }
    }
}

pub enum CasterRef {
    Player,
    Monster(usize),
    // TODO magical entity, item, etc.
}

pub struct Spellbook {
    commands: Vec<Command>,
    spell_table: HashMap<String, usize>,
}

impl Spellbook {
    pub fn empty() -> Spellbook {
        Spellbook {
            commands: Vec::new(),
            spell_table: HashMap::new(),
        }
    }

    pub fn monster_spellbook() -> Spellbook {
        let list: Vec<(Option<&str>, Command)> = cmdlist::CmdListParser::new().parse(MONSTER_SPELLBOOK).unwrap();
        let (labels, commands): (Vec<_>, Vec<_>) = list.into_iter().unzip();
        let mut spell_table: HashMap<String, usize> = HashMap::new();
        for (i, label) in labels.into_iter().enumerate() {
            if let Some(label) = label {
                spell_table.insert(label.to_string(), i);
            }
        }
        let commands = commands.into_iter().map(|cmd| match cmd {
            // TODO handle calls/jump to invalid labels
            Command::CallStr(label) => Command::Call(*spell_table.get(&label).unwrap()),
            Command::JumpStr(label) => Command::Jump(*spell_table.get(&label).unwrap()),
            Command::JumpIfGtStr(a, b, label) => Command::JumpIfGt(a, b, *spell_table.get(&label).unwrap()),
            _ => cmd,
        }).collect();
        Spellbook {
            commands: commands,
            spell_table: spell_table,
        }
    }

    pub fn basic() -> Spellbook {
        let mut spell_table = HashMap::new();
        spell_table.insert("left".to_string(), 0);
        spell_table.insert("right".to_string(), 2);
        spell_table.insert("up".to_string(), 4);
        spell_table.insert("down".to_string(), 6);
        spell_table.insert("wait".to_string(), 8);
        spell_table.insert("attack_left".to_string(), 9);
        spell_table.insert("attack_right".to_string(), 12);
        spell_table.insert("attack_up".to_string(), 15);
        spell_table.insert("attack_down".to_string(), 18);
        Spellbook {
            commands: vec!(
                Command::Move(Direction::Left),
                Command::Return,

                Command::Move(Direction::Right),
                Command::Return,

                Command::Move(Direction::Up),
                Command::Return,

                Command::Move(Direction::Down),
                Command::Return,

                Command::Return,

                Command::MoveCursor(Direction::Left),
                Command::Damage(5),
                Command::Return,

                Command::MoveCursor(Direction::Right),
                Command::Damage(5),
                Command::Return,

                Command::MoveCursor(Direction::Up),
                Command::Damage(5),
                Command::Return,

                Command::MoveCursor(Direction::Down),
                Command::Damage(5),
                Command::Return,

            ),
            spell_table: spell_table,
        }
    }
}

pub enum Command {
    PushVal(i32), 
    PushReg(usize),
    Pop(usize),
    Copy(usize, usize),
    Store(i32, usize),
    Call(usize),
    CallStr(String),
    Return,
    Noop,
    Jump(usize),
    JumpStr(String),
    JumpIfGt(usize, usize, usize), // compare reg, compare reg, destination
    JumpIfGtStr(usize, usize, String), // compare reg, compare reg, destination

    // operators
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
    Sub(usize, usize, usize),
    Div(usize, usize, usize),
    Mod(usize, usize, usize),
    And(usize, usize, usize),
    Or(usize, usize, usize),
    Xor(usize, usize, usize),
    Not(usize, usize),

    // System calls
    PromptDirection, // result in registers x, y
    PromptLocation, // result in registers x, y

    MoveCursor(Direction),
    Damage(usize), // energy
    Move(Direction),
    //Conjure(usize, i32), // spell label, energy -> result in c
    //Launch(usize, usize, usize), // object, x, y

    // Data queries
    QueryEnergy, // result in register e
    QueryLocationSelf, // result in x, y
    QueryLocationCursor, // result in x, y
    QueryValidLocation(usize, usize), // result r (bool)
    QueryPassableLocation(usize, usize), // result r (bool)
    QueryMonsterLocation(usize, usize), // result r (bool)
}

pub struct SpellEngine {
    registers: [i32; 26],
    stack: Vec<i32>,
    call_stack: Vec<usize>,
    pub level: Level,
}

const STACK_SIZE: usize = 1000;

impl SpellEngine {
    pub fn new(level: Level) -> SpellEngine {
        SpellEngine {
            registers: [0; 26],
            stack: Vec::with_capacity(STACK_SIZE),
            call_stack: Vec::with_capacity(STACK_SIZE),
            level: level,
        }
    }

    pub fn do_turn(&mut self, spell: &str, player_spellbook: &Spellbook, monster_spellbook: &Spellbook) -> Option<String> {
        let player_error = self.cast(&CasterRef::Player, player_spellbook, spell);
        if player_error.is_some() {
            player_error
        } else {
            self.level.regen(&CasterRef::Player);
            // Do all other casters turns
            (1..self.level.monsters.len()).filter_map(|index| {
                let caster_ref = CasterRef::Monster(index);
                let spell = self.level.get_spell(&caster_ref);
                let result = self.cast(&caster_ref, monster_spellbook, spell);
                if result.is_none() {
                    self.level.regen(&caster_ref);
                }
                result
            }).next()
        }
    }

    fn cast(&mut self, caster_ref: &CasterRef, spellbook: &Spellbook, spell: &str) -> Option<String> {
        if let Some(i) = spellbook.spell_table.get(spell) {
            let mut result = None;
            let mut instruction_pointer = *i;
            let mut cursor = self.level.location(&caster_ref).clone();
            while result.is_none() {
                if instruction_pointer >= spellbook.commands.len() {
                    return Some("Unexpected end of execution".to_string())
                }
                let cmd = &spellbook.commands[instruction_pointer];
                instruction_pointer += 1;
                result = match cmd {
                    Command::PushVal(val) => if self.stack.len() >= self.stack.capacity() { 
                        Some("Stack overflow".to_string())
                    } else {
                        self.stack.push(*val);
                        None
                    }, 
                    Command::PushReg(reg) => if self.stack.len() >= self.stack.capacity() {
                        Some("Stack overflow".to_string())
                    } else {
                        self.stack.push(self.registers[*reg]);
                        None
                    },
                    Command::Pop(reg) => if let Some(val) = self.stack.pop() {
                        self.registers[*reg] = val;
                        None
                    } else {
                        Some("Called pop on empty stack".to_string())
                    },
                    Command::Copy(src, dest) => {
                        self.registers[*dest] = self.registers[*src];
                        None
                    },
                    Command::Store(val, reg) => {
                        self.registers[*reg] = *val;
                        None
                    },
                    Command::Call(spell) => if self.call_stack.len() >= self.call_stack.capacity() {
                        Some("Call stack overflow".to_string())
                    } else {
                        self.call_stack.push(instruction_pointer);
                        instruction_pointer = *spell;
                        None
                    },
                    Command::Return => if let Some(val) = self.call_stack.pop() {
                        instruction_pointer = val;
                        None 
                    } else {
                        break;
                    },
                    Command::Noop => None,
                    Command::Jump(dest) => if *dest < spellbook.commands.len() {
                        instruction_pointer = *dest;
                        None
                    } else {
                        Some("called jump to invalid location".to_string())
                    },
                    Command::JumpIfGt(a, b, dest) => if *dest < spellbook.commands.len() {
                        if a > b {
                            instruction_pointer = *dest;
                        }
                        None
                    } else {
                        Some("called jumpifgt to invalid location".to_string())
                    },
                    Command::Add(a, b, dest) => {
                        self.registers[*dest] = self.registers[*a] + self.registers[*b];
                        None
                    },
                    Command::Mul(a, b, dest) => {
                        self.registers[*dest] = self.registers[*a] * self.registers[*b];
                        None
                    },
                    Command::Sub(a, b, dest) => {
                        self.registers[*dest] = self.registers[*a] - self.registers[*b];
                        None
                    },
                    Command::Div(a, b, dest) => {
                        self.registers[*dest] = self.registers[*a] / self.registers[*b];
                        None
                    },
                    Command::Mod(a, b, dest) => {
                        self.registers[*dest] = self.registers[*a] % self.registers[*b];
                        None
                    },
                    Command::And(a, b, dest) => {
                        self.registers[*dest] = self.registers[*a] & self.registers[*b];
                        None
                    },
                    Command::Or(a, b, dest) => {
                        self.registers[*dest] = self.registers[*a] | self.registers[*b];
                        None
                    },
                    Command::Xor(a, b, dest) => {
                        self.registers[*dest] = self.registers[*a] ^ self.registers[*b];
                        None
                    },
                    Command::Not(a, dest) => {
                        self.registers[*dest] = !self.registers[*a];
                        None
                    },
                    Command::PromptDirection => Some("Prompting not yet supported".to_string()),
                    Command::PromptLocation => Some("Prompting not yet supported".to_string()),
                    Command::MoveCursor(direction) => {
                        if let Some(loc) = self.level.reify_location(direction.location(), &self.level.location(&caster_ref)) {
                            // TODO cursor move energy cost?
                            if self.level.cast(&caster_ref, 5) {
                                cursor = loc;
                                None
                            } else {
                                Some("Not enough energy to move cursor".to_string())
                            }
                        } else {
                            Some("Invalid location".to_string())
                        }
                    },
                    Command::Damage(register) => {
                        // TODO check sign of energy value
                        let energy = self.registers[*register] as u32;
                        // TODO convert energy to damage
                        if self.level.is_monster(&cursor) {
                            if self.level.cast(&caster_ref, energy) {
                                self.level.damage(&cursor, energy);
                                None
                            } else {
                                Some("Not enough energy to attack".to_string())
                            }
                        } else {
                            Some("Nobdy there to attack".to_string())
                        }
                    },
                    Command::Move(direction) => {
                        if let Some(loc) = self.level.reify_location(direction.location(), &self.level.location(&caster_ref)) {
                            if self.level.is_passable(&loc) && !self.level.is_monster(&loc) {
                                if self.level.cast(&caster_ref, 10) {
                                    // TODO check if valid move
                                    // TODO multiply cost by distance moved or just check that it's
                                    // adjacent?
                                    self.level.move_to(&caster_ref, loc);
                                    None
                                } else {
                                    Some("Not enough energy to move".to_string())
                                }
                            } else {
                                Some("That space is occupied".to_string())
                            }
                        } else {
                            Some("Invalid location".to_string())
                        }
                    },
                    //Command::Conjure(_spell, _energy) => Some("conjuring not yet supported".to_string()),
                    //Command::Launch(_object, _x, _y) => Some("launching not yet supported".to_string()),
                    Command::QueryEnergy => {
                        self.registers[4] = self.level.get_energy(&caster_ref) as i32;
                        None
                    },
                    Command::QueryLocationSelf => {
                        let (x, y) = self.level.location(&caster_ref);
                        self.registers[23] = x as i32;
                        self.registers[24] = y as i32;
                        None
                    },
                    Command::QueryLocationCursor => {
                        self.registers[23] = cursor.0 as i32;
                        self.registers[24] = cursor.1 as i32;
                        None
                    },
                    Command::QueryValidLocation(x_reg, y_reg) => {
                        // TODO check i32 -> isize conversion?
                        let loc = (self.registers[*x_reg] as isize, self.registers[*y_reg] as isize);
                        self.registers[17] = if self.level.reify_location(loc, &(0, 0)).is_some() { 1 } else { 0 };
                        None
                    },
                    Command::QueryPassableLocation(x_reg, y_reg) => {
                        // TODO check i32 -> isize conversion?
                        let rel_loc = (self.registers[*x_reg] as isize, self.registers[*y_reg] as isize);
                        let loc = self.level.reify_location(rel_loc, &(0, 0));
                        if let Some(loc) = loc {
                            self.registers[17] = 
                                if self.level.is_passable(&loc) { 1 } else { 0 };
                        } else {
                            self.registers[17] = 0;
                        }
                        None
                    },
                    Command::QueryMonsterLocation(x_reg, y_reg) => {
                        // TODO check i32 -> isize conversion?
                        let rel_loc = (self.registers[*x_reg] as isize, self.registers[*y_reg] as isize);
                        let loc = self.level.reify_location(rel_loc, &(0, 0));
                        if let Some(loc) = loc {
                            self.registers[17] = 
                                if self.level.is_monster(&loc) { 1 } else { 0 };
                        } else {
                            self.registers[17] = 0;
                        }
                        None
                    },
                    // TODO handle this better
                    Command::CallStr(_) => panic!("Can't execute call str"),
                    Command::JumpStr(_) => panic!("Can't execute jump str"),
                    Command::JumpIfGtStr(_, _, _) => panic!("Can't execute jump str"),
                }
            }
            self.clear();
            result
        } else {
            Some("Unknown spell".to_string())
        }
    }

    fn clear(&mut self) {
        for i in 0..26 {
            self.registers[i] = 0;
        }
        self.stack.clear();
        self.call_stack.clear();
    }
}


