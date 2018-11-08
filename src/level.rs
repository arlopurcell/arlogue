use tile::Tile;
use monster::Monster;

pub struct Level {
    pub terrain: [[Tile; 20]; 20],
    pub monster_list: MonsterList,
}

pub struct MonsterList {
    monsters: Vec<Monster>,
    locations: Vec<(usize, usize)>, // column, row
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

impl MonsterList {
    fn new() -> MonsterList {
        MonsterList {
            monsters: Vec::new(),
            locations: Vec::new(),
        }
    }

    fn with_monster_locations<T>(monster_locations: T) -> MonsterList
        where T: IntoIterator<Item = (Monster, (usize, usize))>, 
    {
        let (monsters, locations) = monster_locations.into_iter().unzip();
        MonsterList {
            monsters: monsters,
            locations: locations,
        }
    }

    fn add(&mut self, monster: Monster, column: usize, row: usize) {
        self.monsters.push(monster);
        self.locations.push((column, row));
    }

    pub fn get_monster_locations(&self) -> Vec<(&Monster, &(usize, usize))> {
        self.monsters.iter().zip(self.locations.iter()).collect()
    }

    pub fn player_location(&self) -> (usize, usize) {
        self.locations[0]
    }
}

impl Level {
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
        let (old_col, old_row) = self.monster_list.locations[index];
        self.move_monster_to(
            index, 
            (old_col as isize + col_change) as usize, 
            (old_row as isize + row_change) as usize,
        )
    }

    fn move_monster_to(&mut self, index: usize, new_col: usize, new_row: usize) -> bool {
        let available = !self.terrain[new_col][new_row].is_wall 
            && !self.monster_list.locations.iter().any(|(col, row)| { *col == new_col && *row == new_row });
        if available {
            self.monster_list.locations[index].0 = new_col;
            self.monster_list.locations[index].1 = new_row;
            true
        } else {
            false
        }
    }

    pub fn stupid() -> Level {
        Level {
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
            monster_list: MonsterList::with_monster_locations(vec!(
                (Monster::player(), (7, 7)), 
                (Monster::ant(), (12, 12)),
            )),
        }
    }
}
