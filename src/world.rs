use std::mem;
use quicksilver::{
    Future,
    Result,
    lifecycle::{State, Window, Asset, Event},
    graphics::{Image, Color, Background::Img, View},
    geom::{Rectangle, Vector, Shape},
    input::{Key, ButtonState}
};

use level::{
    Level,
    Action::*,
    Direction::*,
};
use monster::Monster;

pub const SCREEN_SIZE: Vector = Vector {x: 800.0, y: 600.0};

struct SpriteSheet {
    image: Image,
    sprite_size: Vector,
    rows: usize,
    columns: usize,
}

impl SpriteSheet {
    fn get(&self, index: usize) -> Image {
        self.image.subimage(Rectangle::new(Vector::new((index % self.columns) as u32, (index / self.columns) as u32).times(self.sprite_size), self.sprite_size))
    }
}

pub struct World {
    current_level: Level,
    // TODO other saved levels
    sprite_sheet: Asset<SpriteSheet>,
}

impl State for World {
    fn new() -> Result<World> {
        let image = Image::load("nethack3.6.1tiles32.png");
        Ok(World{
            current_level: Level::stupid(),
            sprite_sheet: Asset::new(image.map(|image| {
                SpriteSheet {
                    image: image,
                    sprite_size: Vector::new(32, 32),
                    rows: 37,
                    columns: 40,
                }
            })),
        })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        let level = &self.current_level;
        self.sprite_sheet.execute(|sprite_sheet| {
            let player_location = level.monster_list.player_location();
            window.set_view(View::new(Rectangle::new(
                        Vector::new(player_location.0 as f32, player_location.1 as f32).times(sprite_sheet.sprite_size) - (SCREEN_SIZE / 2),
                        SCREEN_SIZE
            )));
            for row in 0..20 {
                for col in 0..20 {
                    let tile_rect = Rectangle::new(Vector::new((32 * col) as u32, (32 * row) as u32), sprite_sheet.sprite_size);
                    if let Some(sprite_index) = level.terrain[col][row].sprite_index{
                        let tile_img = &sprite_sheet.get(sprite_index);
                        window.draw(&tile_rect, Img(tile_img));
                    }
                }
            }
            for (monster, (col, row)) in level.monster_list.get_monster_locations() {
                let tile_rect = Rectangle::new(Vector::new((32 * col) as u32, (32 * row) as u32), sprite_sheet.sprite_size);
                let monster_img = &sprite_sheet.get(monster.sprite_index);
                window.draw(&tile_rect, Img(monster_img));
            }
            Ok(())
        })
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match event {
            Event::Key(key, state) => match state {
                ButtonState::Pressed => match key {
                    Key::Left => self.current_level.do_turn(Move(Left)),
                    Key::Right => self.current_level.do_turn(Move(Right)),
                    Key::Up => self.current_level.do_turn(Move(Up)),
                    Key::Down => self.current_level.do_turn(Move(Down)),
                    // TODO diagonals
                    Key::Period => self.current_level.do_turn(Wait),
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        };
        Ok(())
    }
}

