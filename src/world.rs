use quicksilver::{
    Future,
    Result,
    lifecycle::{State, Window, Asset, Event},
    graphics::{Image, Color, Background::Img, View, Font, FontStyle},
    geom::{Rectangle, Vector, Shape},
    input::{Key, ButtonState}
};

use crate::level::{
    Level,
    Action::*,
    Direction::*,
};
use crate::spells::{SpellEngine, Spellbook, CasterRef};

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
    // TODO saved levels
    sprite_sheet: Asset<SpriteSheet>,
    font: Asset<Font>,
    message: Option<String>,
    spell_engine: SpellEngine,
    player_spellbook: Spellbook,
    monster_spellbook: Spellbook,
}

impl State for World {
    fn new() -> Result<World> {
        let image = Image::load("nethack3.6.1tiles32.png");
        Ok(World{
            spell_engine: SpellEngine::new(Level::stupid()),
            player_spellbook: Spellbook::basic(),
            monster_spellbook: Spellbook::basic(),
            font: Asset::new(Font::load("font.ttf")),
            message: None,
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
        let message = &self.message;
        self.font.execute(|font| {
            if let Some(msg) = message {
                let style = FontStyle::new(24.0, Color::WHITE);
                let image = font.render(&msg, &style)?;
                window.set_view(View::new(Rectangle::new(Vector::new(0, 0), SCREEN_SIZE)));
                window.draw(&Rectangle::new(Vector::new(0, 0), image.area().size()), Img(&image));
            }
            Ok(())
        })?;

        let level = &self.spell_engine.level;
        self.sprite_sheet.execute(|sprite_sheet| {
            let (col, row) = level.monsters[0].location();
            window.set_view(View::new(Rectangle::new(
                        Vector::new(col as f32, row as f32).times(sprite_sheet.sprite_size) - (SCREEN_SIZE / 2),
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
            for monster in level.monsters.iter() {
                let (col, row) = monster.location();
                let tile_rect = Rectangle::new(Vector::new((32 * col) as u32, (32 * row) as u32), sprite_sheet.sprite_size);
                let monster_img = &sprite_sheet.get(monster.sprite_index);
                window.draw(&tile_rect, Img(monster_img));
            }
            Ok(())
        })?;
        Ok(())
    }

    fn event(&mut self, event: &Event, _window: &mut Window) -> Result<()> {
        match event {
            Event::Key(key, state) => match state {
                ButtonState::Pressed => {
                    /*
                    let error_msg = match key {
                        Key::Left => self.spell_engine.cast(CasterRef::Monster(0), &self.player_spellbook, "left"),
                        Key::Right => self.spell_engine.cast(CasterRef::Monster(0), &self.player_spellbook, "right"),
                        Key::Up => self.spell_engine.cast(CasterRef::Monster(0), &self.player_spellbook, "up"),
                        Key::Down => self.spell_engine.cast(CasterRef::Monster(0), &self.player_spellbook, "down"),
                        // TODO diagonals
                        Key::Left => self.spell_engine.cast(CasterRef::Monster(0), &self.player_spellbook, "wait"),
                        _ => None,
                    };
                    */
                    let spell = match key {
                        Key::Left => Some("left"),
                        Key::Right => Some("right"),
                        Key::Up => Some("up"),
                        Key::Down => Some("down"),
                        // TODO diagonals
                        Key::Left => Some("wait"),
                        _ => None,
                    };
                    if let Some(spell) = spell {
                        self.message = self.spell_engine.do_turn(spell, &self.player_spellbook, &self.monster_spellbook);
                    }
                    ()
                },
                _ => (),
            },
            _ => (),
        };
        Ok(())
    }
}


