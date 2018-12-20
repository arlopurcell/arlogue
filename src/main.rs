extern crate quicksilver;

#[macro_use] extern crate lalrpop_util;

use quicksilver::{
    lifecycle::{Settings, run},
};

mod tile;
mod monster;
mod level;
mod world;
mod spells;
mod utils;
mod constants;

use crate::world::SCREEN_SIZE;

fn main() {
    run::<world::World>("Arlogue", SCREEN_SIZE, Settings::default());
}
