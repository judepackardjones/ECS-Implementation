mod ecs;
mod sparseset;
mod entity_storage;

#[cfg(test)]
mod tests;

use std::{any::{Any, TypeId}, cell::{RefCell}, collections::HashMap};
use crate::{ecs::{components::*, systems::*}, sparseset::*, entity_storage::*};
use macroquad::prelude::*;


fn main() {
    let mut entity_manager = EntityManager::new(); // Create Entity Manager 

    let mut world: World  = World::new(); // Create world and attach the entity manager 
    
    world.ensure_set::<Position>(); // Ensures that world has a sparseset for position, and if it doesn't, creates one 
    world.ensure_set::<Size>();
    world.ensure_set::<Player>();

    let player_id = entity_manager.get_fresh_id();
    world.add_entity_component(player_id.clone(), Player {});
    world.add_entity_component(player_id.clone(), Position {x: 50, y: 50});
    world.add_entity_component(player_id.clone(), Size {r: 10});

    let wall_id = entity_manager.get_fresh_id();
    world.add_entity_component(wall_id.clone(), Position {x: 0, y: 0});
    world.add_entity_component(wall_id.clone(), Size {r: 30});
    
    world.remove_entity_component::<Position>(&wall_id);

    let size_player_id = entity_manager.get_fresh_id();
    world.add_entity_component(size_player_id.clone(), Player {});
    world.add_entity_component(size_player_id.clone(), Size {r: 30});
    {
    let component_vec = world.get_mut_component_vec::<Position>();
    for i in component_vec {
        i.x += 1;
    }
    }
    println!("{:?}", world.get_mut_component_vec::<Position>());
}


// #[macroquad::main("GOL")]
// async fn main() {

//     loop {
//         dispatch();
//         next_frame().await
//     }
// }

fn dispatch() { // Should control the ordering of systems (Draw should be last)

    // DrawSystem::run()
}