use crate::tile::Tile;
use crate::monster::{Monster, MonsterType};
use crate::spells::{CasterRef, Caster};
use crate::utils::Location;

const LEVEL_SIZE: usize = 20;

pub struct Level {
    pub terrain: [[Tile; LEVEL_SIZE]; LEVEL_SIZE],
    pub monsters: Vec<Monster>,
}

pub enum Action {
    Wait,
    Move(Direction),
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Level {
    pub fn cast(&mut self, caster_ref: &CasterRef, cost: u32) -> bool { // false if not enough energy
        self.caster_mut(caster_ref).cast(cost)
    }

    pub fn move_to(&mut self, caster_ref: &CasterRef, location: Location) { self.caster_mut(caster_ref).move_to(location) }

    pub fn regen(&mut self, caster_ref: &CasterRef) {
        self.caster_mut(caster_ref).regen()
    }

    pub fn location(&self, caster_ref: &CasterRef) -> Location {
        self.caster(caster_ref).location
    }

    pub fn get_spell(&self, caster_ref: &CasterRef) -> &'static str {
        // TODO
        "right"
    }

    fn caster(&self, caster_ref: &CasterRef) -> &Caster {
        match caster_ref {
            CasterRef::Player => &self.monsters[0].caster,
            CasterRef::Monster(index) => &self.monsters[*index].caster,
        }
    }

    fn caster_mut(&mut self, caster_ref: &CasterRef) -> &mut Caster {
        match caster_ref {
            CasterRef::Player => &mut self.monsters[0].caster,
            CasterRef::Monster(index) => &mut self.monsters[*index].caster,
        }
    }

    pub fn is_available(&self, location: (isize, isize)) -> bool {
        let (col, row) = location;
        if col < 0 || row < 0 || col >= LEVEL_SIZE as isize || row >= LEVEL_SIZE as isize {
            false
        } else {
            !self.terrain[col as usize][row as usize].is_wall 
                && !self.monsters.iter().any(|monster| { monster.location() == (col as usize, row as usize) })
        }
    }

    /*
    pub fn do_turn(&mut self, action: Action) {
        let did_action = match action {
            Action::Move(dir) => self.move_monster(0, dir),
            Action::Wait => true,
        };
        if did_action {
            // TODO other monsters' turns
        }
    }

    fn move_monster(&mut self, index: usize, dir: Direction) -> bool {
        match dir {
            Direction::Left => self.move_monster_by(index, -1, 0),
            Direction::Right => self.move_monster_by(index, 1, 0),
            Direction::Up => self.move_monster_by(index, 0, -1),
            Direction::Down => self.move_monster_by(index, 0, 1),
            Direction::UpLeft => self.move_monster_by(index, -1, -1),
            Direction::UpRight => self.move_monster_by(index, 1, -1),
            Direction::DownLeft => self.move_monster_by(index, -1, 1),
            Direction::DownRight => self.move_monster_by(index, 1, 1),
        }
    }

    fn move_monster_by(&mut self, index: usize, col_change: isize, row_change: isize) -> bool {
        if let Some((old_col, old_row)) = self.monster_list.locations[index] {
            self.move_monster_to(
                index, 
                (old_col as isize + col_change) as usize, 
                (old_row as isize + row_change) as usize,
                )
        } else {
            false
        }
    }

    fn move_monster_to(&mut self, index: usize, new_col: usize, new_row: usize) -> bool {
        let available = !self.terrain[new_col][new_row].is_wall 
            && !self.monster_list.locations.iter().filter_map(|&l| {l}).any(|(col, row)| { col == new_col && row == new_row });
        if available {
            self.monster_list.locations[index] = Some((new_col, new_row));
            true
        } else {
            false
        }
    }
    */

    pub fn stupid() -> Level {
        Level {
            /*
            monster_list: MonsterList::with_monster_locations(vec!(
                (MonsterType::Player, (7, 7)), 
                (MonsterType::Ant, (12, 12)),
            )),
            */
            monsters: vec!(Monster::player((7, 7)), Monster::ant((12, 12))),
            terrain: [
                [Tile::nothing(); 20],
                [Tile::nothing(); 20],
                [
                    Tile::nothing(),
                    Tile::nothing(),
                    Tile::top_left_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::bot_left_wall(),
                    Tile::nothing(),
                    Tile::nothing(),
                ],
                [
                    Tile::nothing(),
                    Tile::nothing(),
                    Tile::horiz_wall(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::horiz_wall(),
                    Tile::nothing(),
                    Tile::nothing(),
                ],
                [
                    Tile::nothing(),
                    Tile::nothing(),
                    Tile::horiz_wall(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::horiz_wall(),
                    Tile::nothing(),
                    Tile::nothing(),
                ],
                [
                    Tile::nothing(),
                    Tile::nothing(),
                    Tile::horiz_wall(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::horiz_wall(),
                    Tile::nothing(),
                    Tile::nothing(),
                ],
                [
                    Tile::nothing(),
                    Tile::nothing(),
                    Tile::horiz_wall(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::horiz_wall(),
                    Tile::nothing(),
                    Tile::nothing(),
                ],
                [
                    Tile::nothing(),
                    Tile::nothing(),
                    Tile::horiz_wall(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::horiz_wall(),
                    Tile::nothing(),
                    Tile::nothing(),
                ],
                [
                    Tile::nothing(),
                    Tile::nothing(),
                    Tile::horiz_wall(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::horiz_wall(),
                    Tile::nothing(),
                    Tile::nothing(),
                ],
                [
                    Tile::nothing(),
                    Tile::nothing(),
                    Tile::horiz_wall(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::horiz_wall(),
                    Tile::nothing(),
                    Tile::nothing(),
                ],
                [
                    Tile::nothing(),
                    Tile::nothing(),
                    Tile::horiz_wall(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::horiz_wall(),
                    Tile::nothing(),
                    Tile::nothing(),
                ],
                [
                    Tile::nothing(),
                    Tile::nothing(),
                    Tile::horiz_wall(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::horiz_wall(),
                    Tile::nothing(),
                    Tile::nothing(),
                ],
                [
                    Tile::nothing(),
                    Tile::nothing(),
                    Tile::horiz_wall(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::horiz_wall(),
                    Tile::nothing(),
                    Tile::nothing(),
                ],
                [
                    Tile::nothing(),
                    Tile::nothing(),
                    Tile::horiz_wall(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::horiz_wall(),
                    Tile::nothing(),
                    Tile::nothing(),
                ],
                [
                    Tile::nothing(),
                    Tile::nothing(),
                    Tile::horiz_wall(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::horiz_wall(),
                    Tile::nothing(),
                    Tile::nothing(),
                ],
                [
                    Tile::nothing(),
                    Tile::nothing(),
                    Tile::horiz_wall(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::horiz_wall(),
                    Tile::nothing(),
                    Tile::nothing(),
                ],
                [
                    Tile::nothing(),
                    Tile::nothing(),
                    Tile::horiz_wall(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::floor(),
                    Tile::horiz_wall(),
                    Tile::nothing(),
                    Tile::nothing(),
                ],
                [
                    Tile::nothing(),
                    Tile::nothing(),
                    Tile::top_right_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::vert_wall(),
                    Tile::bot_right_wall(),
                    Tile::nothing(),
                    Tile::nothing(),
                ],
                [Tile::nothing(); 20],
                [Tile::nothing(); 20],
            ],
        }
    }
}
