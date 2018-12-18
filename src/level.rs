use crate::tile::Tile;
use crate::monster::{Monster, MonsterType};
use crate::spells::{CasterRef, Caster};
use crate::utils::Location;

const LEVEL_SIZE: usize = 20;

pub struct Level {
    pub terrain: [[Tile; LEVEL_SIZE]; LEVEL_SIZE],
    pub monsters: Vec<Monster>,
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
        match caster_ref {
            CasterRef::Player => "wait",
            CasterRef::Monster(index) => {
                let (my_x, my_y) = self.location(caster_ref);
                let (player_x, player_y) = self.location(&CasterRef::Player);
                if player_x > my_x {
                    if player_y > my_y { "wait" }
                    else if player_y < my_y { "wait" } 
                    else { "right" }
                } else if player_x < my_x {
                    if player_y > my_y { "wait" }
                    else if player_y < my_y { "wait" } 
                    else { "left" }
                } else {
                    if player_y > my_y { "down" }
                    else if player_y < my_y { "up" } 
                    else { "wait" }
                }

            },
        }
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
