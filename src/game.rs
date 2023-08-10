use anyhow::Result;
use hecs::{Entity, World};

use crate::component::Spatial;

pub struct State {
    pub world: World,
    pub player: Entity,
}

pub fn main_loop() -> Result<()> {
    let mut world = World::new();
    let player = world.spawn((Spatial::default(),));

    let mut _state = State { world, player };

    // TODO main game loop

    Ok(())
}
