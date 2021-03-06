use crate::spells::Caster;
use crate::utils::AbsoluteLocation;

const MAX_PER_LEVEL: usize = 200;

pub struct Monster {
    pub stats: StatBlock,
    pub caster: Caster,
    pub sprite_index: usize,
    pub mtype: MonsterType,
}

#[derive(Copy, Clone)]
pub struct StatBlock {
    pub max_hp: u32,
    pub current_hp: u32,
    pub ac: u8,
    // TODO more stuff, resistances, etc.
}

#[derive(Copy, Clone)]
pub enum MonsterType {
    Player,
    Ant,
}

impl Monster {
    pub fn player(location: AbsoluteLocation) -> Monster {
        Monster {
            stats: StatBlock {
                max_hp: 20,
                current_hp: 20,
                ac: 10,
            },
            caster: Caster::simple(location, 10),
            sprite_index: 348,
            mtype: MonsterType::Player,
        }
    }

    pub fn ant(location: AbsoluteLocation) -> Monster {
        Monster {
            stats: StatBlock {
                max_hp: 10,
                current_hp: 10,
                ac: 10,
            },
            caster: Caster::simple(location, 10),
            sprite_index: 0,
            mtype: MonsterType::Ant,
        }
    }

    pub fn location(&self) -> AbsoluteLocation {
        self.caster.location
    }
}
