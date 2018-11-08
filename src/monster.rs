#[derive(Copy, Clone)]
pub struct Monster {
    pub sprite_index: usize,
    // TODO more stuff, max hp, current hp, AC, resistances, etc.
}

impl Monster {
    pub fn player() -> Monster {
        Monster {sprite_index: 348}
    }

    pub fn ant() -> Monster {
        Monster {sprite_index: 0}
    }

}
