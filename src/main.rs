extern crate quicksilver;

use quicksilver::{
    lifecycle::{Settings, run},
};

mod tile;
mod monster;
mod level;
mod world;
mod spells;
mod utils;

use crate::world::SCREEN_SIZE;

fn main() {
    run::<world::World>("Arlogue", SCREEN_SIZE, Settings::default());
}
