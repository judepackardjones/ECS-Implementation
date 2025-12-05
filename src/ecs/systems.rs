use macroquad::prelude::*;
use crate::ecs::{components::*, entities::*};

// The concrete World type is not imported here (no `crate::main` module in crate root),
// so accept a generic placeholder for the world parameter to keep this module independent.
pub struct DrawSystem;
pub struct CreateEntitiesSystem;

pub trait System {
    fn run();
}

