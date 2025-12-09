mod ecs;
mod sparseset;
mod entity_storage;

#[cfg(test)]
mod tests;

use std::{any::{Any, TypeId}, cell::RefCell, collections::HashMap};
use crate::{ecs::{components::*, systems::*}, sparseset::*, entity_storage::*};
use macroquad::prelude::*;


fn main() {
    let mut entity_manager = EntityManager::new(); // Create Entity Manager 

    let mut world: World  = World::new(); // Create world and attach the entity manager 
    
    world.ensure_set::<Position>(); // Ensures that world has a sparseset for position, and if it doesn't, creates one 

    let first_entity_id = entity_manager.get_fresh_id();
    entity_manager.delete_id(&first_entity_id);
    let second_entity_id = entity_manager.get_fresh_id();

    let _ = world.get_component_set::<Position>().unwrap().add_component(second_entity_id.clone(), Position {x: 50, y: 60});

    let second_add = world.get_component_set::<Position>().unwrap().add_component(second_entity_id.clone(), Position {x: 30, y: 50}).unwrap_err(); // Intended to fail 
    println!("{}", second_add); // Intended to fail as Component already exists
    let first_add = world.get_component_set::<Position>().unwrap().add_component(first_entity_id.clone(), Position {x: 10, y: 50}).unwrap_err(); // Intended to fail 
    println!("{}", first_add); // Intended to fail as stale ID 

    let second_entity_position = world.get_component_set::<Position>().unwrap().get_component(&second_entity_id).unwrap();
    println!("Id: {} Generation: {}, Position Component: {:?}", &second_entity_id.index, &second_entity_id.generation, &second_entity_position);
    
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