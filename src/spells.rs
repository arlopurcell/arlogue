use std::collections::HashMap;
use std::ops::DerefMut;

use crate::level::Level;
use crate::utils::{Location, Direction};
use crate::monster::Monster;

pub struct Caster {
    pub location: Location,
    energy: u32,
    max_energy: u32,
    energy_regen: u32,
}

impl Caster {
    pub fn simple(location: Location, energy: u32) -> Caster {
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

    pub fn move_to(&mut self, location: Location) { self.location = location }

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

    pub fn basic() -> Spellbook {
        let mut spell_table = HashMap::new();
        spell_table.insert("left".to_string(), 0);
        spell_table.insert("right".to_string(), 2);
        spell_table.insert("up".to_string(), 4);
        spell_table.insert("down".to_string(), 6);
        spell_table.insert("wait".to_string(), 8);
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
            ),
            spell_table: spell_table,
        }
    }
}

enum Command {
    PushVal(i32), 
    PushReg(usize),
    Pop(usize),
    Copy(usize, usize),
    Store(i32, usize),
    Call(usize),
    Return,
    Noop,
    Jump(usize),
    JumpIfGt(usize, usize, usize), // compare reg, compare reg, destination

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

    Damage(Vec<(usize, usize, u32)>), // x, y, energy
    Move(Direction), // x, y (relative)
    Conjure(usize, u32), // spell label, energy -> result in c
    Launch(usize, usize, usize), // object, x, y
}

pub struct SpellEngine {
    registers: [i32; 26],
    stack: Vec<i32>,
    call_stack: Vec<usize>,
    pub level: Level,
    player_spellbook: Spellbook,
    monster_spellbook: Spellbook,
}

const STACK_SIZE: usize = 1000;

impl SpellEngine {
    pub fn new(level: Level) -> SpellEngine {
        SpellEngine {
            registers: [0; 26],
            stack: Vec::with_capacity(STACK_SIZE),
            call_stack: Vec::with_capacity(STACK_SIZE),
            level: level,
            player_spellbook: Spellbook::basic(),
            monster_spellbook: Spellbook::basic(),
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
                    Command::PromptLocation => Some("Prompting not yet supported".to_string()),
                    Command::Damage(_targets) => {
                        // TODO change to single target
                        Some("Damage not done yet".to_string())
                    },
                    Command::Move(direction) => {
                        let (old_col, old_row) = self.level.location(&caster_ref);
                        let (x, y) = match direction {
                            Direction::Left => (-1, 0),
                            Direction::Right => (1, 0),
                            Direction::Up => (0, -1),
                            Direction::Down => (0, 1),
                            Direction::UpLeft => (-1, -1),
                            Direction::UpRight => (1, -1),
                            Direction::DownLeft => (-1, 1),
                            Direction::DownRight => (1, 1),
                        };
                            
                        let loc = (old_col as isize + x, old_row as isize + y);
                        
                        if self.level.is_available(loc) {
                            if self.level.cast(&caster_ref, 10) {
                                // TODO check if valid move
                                // TODO multiply cost by distance moved or just check that it's
                                // adjacent?
                                self.level.move_to(&caster_ref, (loc.0 as usize, loc.1 as usize));
                                None
                            } else {
                                Some("Not enough energy to move".to_string())
                            }
                        } else {
                            Some("That space is occupied".to_string())
                        }
                    },
                    Command::Conjure(_spell, _energy) => Some("conjuring not yet supported".to_string()),
                    Command::Launch(_object, _x, _y) => Some("launching not yet supported".to_string()),
                }
            }
            result
        } else {
            Some("Unknown spell".to_string())
        }
    }
}


