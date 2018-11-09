const MAX_PER_LEVEL: usize = 200;

pub struct MonsterList {
    size: usize,
    stat_blocks: [Option<StatBlock>; MAX_PER_LEVEL],
    sprite_indices: [Option<usize>; MAX_PER_LEVEL],
    pub locations: [Option<(usize, usize)>; MAX_PER_LEVEL], // column, row
    types: [MonsterType; MAX_PER_LEVEL],
}

#[derive(Copy, Clone)]
pub struct StatBlock {
    pub max_hp: u16,
    pub current_hp: u16,
    pub ac: u8,
    // TODO more stuff, resistances, etc.
}

#[derive(Copy, Clone)]
pub enum MonsterType {
    None,
    Player,
    Ant,
}

pub struct SpriteLocations<'a> {
    counter: usize,
    monster_list: &'a MonsterList,
}

impl <'a> Iterator for SpriteLocations<'a> {
    type Item = (usize, (usize, usize)); // sprite index, (column, row)

    fn next(&mut self) -> Option<(usize, (usize, usize))> {
        if self.counter < self.monster_list.size {
            if let Some(sprite_index) = self.monster_list.sprite_indices[self.counter] {
                if let Some(location) = self.monster_list.locations[self.counter] {
                    self.counter += 1;
                    Some((sprite_index, location))
                } else { None }
            } else { None }
        } else { None }
    }
}

impl MonsterList {
    pub fn with_monster_locations<T>(monster_locations: T) -> MonsterList
        where T: IntoIterator<Item = (MonsterType, (usize, usize))>, 
    {
        let mut ml = MonsterList {
            size: 0,
            stat_blocks: [None; MAX_PER_LEVEL],
            sprite_indices: [None; MAX_PER_LEVEL],
            locations: [None; MAX_PER_LEVEL],
            types: [MonsterType::None; MAX_PER_LEVEL],
        };
        for (mtype, (col, row)) in monster_locations.into_iter() {
            ml.add_with_type(col, row, mtype);
        }
        ml
    }

    pub fn sprite_locations(&self) -> SpriteLocations {
        SpriteLocations {
            counter: 0,
            monster_list: self,
        }
    }

    pub fn player_location(&self) -> Option<(usize, usize)> {
        let maybe_player_index = self.types.iter().position(|&mt| {
            if let MonsterType::Player = mt { true } else { false }
        });
        if let Some(player_index) = maybe_player_index {
            self.locations[player_index]
        } else {
            None
        }
    }

    fn add(&mut self, column: usize, row: usize, stat_block: StatBlock, sprite_index: usize, mtype: MonsterType) {
        if (size < MAX_PER_LEVEL) {
            self.stat_blocks[self.size] = Some(stat_block);
            self.sprite_indices[self.size] = Some(sprite_index);
            self.locations[self.size] = Some((column, row));
            self.types[self.size] = mtype;
            self.size += 1;
        }
    }

    pub fn add_with_type(&mut self, column: usize, row: usize, mtype: MonsterType) {
        let si_sb = match mtype {
            MonsterType::Player => Some((348, StatBlock {
                max_hp: 20,
                current_hp: 20,
                ac: 10,
            })),
            MonsterType::Ant => Some((0, StatBlock {
                max_hp: 10,
                current_hp: 10,
                ac: 10,
            })),
            MonsterType::None => None,
        };
        if let Some((sprite_index, stat_block)) = si_sb {
            self.add(column, row, stat_block, sprite_index, mtype)
        }
    }
}
