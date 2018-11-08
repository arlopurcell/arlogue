#[derive(Copy, Clone)]
pub struct Tile {
    pub sprite_index: Option<usize>,
    pub is_wall: bool,
    // TODO add more stuff like passable, etc.
}

impl Tile {
    pub fn nothing() -> Tile {
        Tile {
            sprite_index: None,
            is_wall: true,
        }
    }

    pub fn floor() -> Tile {
        Tile {
            sprite_index: Some(870),
            is_wall: false,
        }
    }

    pub fn vert_wall() -> Tile {
        Tile {
            sprite_index: Some(851),
            is_wall: true,
        }
    }

    pub fn horiz_wall() -> Tile {
        Tile {
            sprite_index: Some(852),
            is_wall: true,
        }
    }

    pub fn top_left_wall() -> Tile {
        Tile {
            sprite_index: Some(853),
            is_wall: true,
        }
    }

    pub fn top_right_wall() -> Tile {
        Tile {
            sprite_index: Some(854),
            is_wall: true,
        }
    }

    pub fn bot_left_wall() -> Tile {
        Tile {
            sprite_index: Some(855),
            is_wall: true,
        }
    }

    pub fn bot_right_wall() -> Tile {
        Tile {
            sprite_index: Some(856),
            is_wall: true,
        }
    }
    
    // TODO
}
   

