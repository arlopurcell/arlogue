// Draw some multi-colored geometry to the screen
extern crate quicksilver;

use quicksilver::{
    lifecycle::{Settings, run},
};

mod tile;
mod monster;
mod level;
mod world;

use world::SCREEN_SIZE;

fn main() {
    run::<world::World>("Arlogue", SCREEN_SIZE, Settings::default());
}
