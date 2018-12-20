use quicksilver::{
    Future,
    Result,
    lifecycle::{State, Window, Asset, Event},
    graphics::{Image, Color, Background::Img, View, Font, FontStyle, Mesh, Drawable, Background::Col},
    geom::{Rectangle, Vector, Shape, Transform},
    input::{Key, ButtonState}
};

use crate::level::Level;
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
    assets: Asset<(SpriteSheet, Font)>,
    mesh: Mesh,
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
            message: None,
            assets: Asset::new(image.map(|image| {
                SpriteSheet {
                    image: image,
                    sprite_size: Vector::new(32, 32),
                    rows: 37,
                    columns: 40,
                }
            }).join(Font::load("font.ttf"))
            ),
            mesh: Mesh::new(),
        })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let message = &self.message;
        let level = &self.spell_engine.level;
        let mut mesh = &mut self.mesh;
        mesh.clear();
        self.assets.execute(|(sprite_sheet, font)| {
            if let Some(msg) = message {
                let style = FontStyle::new(24.0, Color::WHITE);
                let image = font.render(&msg, &style)?;
                Rectangle::new(Vector::new(0, 0), image.area().size()).draw(mesh, Img(&image), Transform::IDENTITY, 20);
            }

            let camera = {
                let (col, row) = level.monsters[0].location();
                Transform::translate(
                    (Vector::new(col as f32, row as f32).times(sprite_sheet.sprite_size) - (SCREEN_SIZE / 2)) * -1
                )
            };

            // TODO this can be rendered on its own mesh and only drawn when the level is loaded
            for row in 0..20 {
                for col in 0..20 {
                    let tile_rect = Rectangle::new(Vector::new((32 * col) as u32, (32 * row) as u32), sprite_sheet.sprite_size);
                    if let Some(sprite_index) = level.terrain[col][row].sprite_index{
                        let tile_img = &sprite_sheet.get(sprite_index);
                        tile_rect.draw(mesh, Img(tile_img), camera, 0);
                    }
                }
            }
            for monster in level.monsters.iter() {
                let (col, row) = monster.location();
                let monster_rect = Rectangle::new(Vector::new((32 * col) as u32, (32 * row) as u32), sprite_sheet.sprite_size);
                let monster_img = &sprite_sheet.get(monster.sprite_index);
                monster_rect.draw(mesh, Img(monster_img), camera, 10);
            }
            Ok(())
        })?;
        window.clear(Color::BLACK)?;
        window.mesh().extend(mesh);
        Ok(())
    }

    fn event(&mut self, event: &Event, _window: &mut Window) -> Result<()> {
        match event {
            Event::Key(key, state) => match state {
                ButtonState::Pressed => {
                    let spell = match key {
                        Key::Left => Some("left"),
                        Key::Right => Some("right"),
                        Key::Up => Some("up"),
                        Key::Down => Some("down"),
                        // TODO diagonals
                        Key::Period => Some("wait"),

                        Key::A => Some("attack_left"),
                        Key::D => Some("attack_right"),
                        Key::W => Some("attack_up"),
                        Key::S => Some("attack_down"),
                        // TODO diagonals
                        // TODO
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



