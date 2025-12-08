mod ecs;
mod sparseset;

use std::{any::{Any, TypeId}, cell::RefCell, collections::HashMap};
use crate::{ecs::{components::*, systems::*}, sparseset::*};
use macroquad::prelude::*;

#[derive(Copy, Clone)]
pub struct EntityId {
    index: u32,
    generation: u32,
}

pub struct EntityManager {
    generations: Vec<u32>, // The generation[EntityId.index] = Next valid generation
    free_indices_queue: Vec<u32>, // Freed indices that can be used later. 
    next_index: u32, // If the queue is empty, get the next highest index.  
}
pub struct World{
    pub items: HashMap<TypeId, Box<dyn AnySparseSet>>, // collections cannot have more than one generic type so we have to make a trait object for sparseset
    pub manager: EntityManager,
}

impl EntityManager {
    fn new() -> Self {
        EntityManager {
            generations: Vec::new(), 
            free_indices_queue: Vec::new(),
            next_index: 0,
        }
    }
    fn generate_new_id(&mut self) -> EntityId {
        if self.free_indices_queue.is_empty() { // If there is no waiting index, increment the next_index 
            self.generations.push(1); // Add a new element for generations that will always be there 
            let given_index = self.next_index; 
            self.next_index += 1;
            EntityId {index: given_index, generation: 0}
    
        } else {
            let new_index = self.free_indices_queue.pop().unwrap();
            EntityId {index: new_index, generation: self.generations[new_index as usize]}
        }
    }
    fn delete_id(&mut self, id: &EntityId) {
        self.generations[id.index as usize] += 1;
        self.free_indices_queue.push(id.index);
    }
    pub fn check_alive(&self, id: &EntityId) -> bool {
        let index = id.index as usize;
        if index >= self.generations.len() {
            return false;
        }
        id.generation + 1 == self.generations[index]
}
}


impl World {
    fn new(entity_manager: EntityManager) -> Self{
        World {
            items: HashMap::new(),
            manager: entity_manager,
        }
    }
    fn ensure_set<C: Component>(&mut self) {
        let type_id = TypeId::of::<C>();
        if !self.items.contains_key(&type_id) {
            self.items.insert(type_id, Box::new(SparseSet::<C>::new()));
        }
    }
    fn get_fresh_id(&mut self) -> EntityId {
        self.manager.generate_new_id()
    }
    
    fn remove_entity(&mut self, id: &EntityId) {
        self.manager.delete_id(id);
        for (k, v) in &self.items {

        }
        // Add removing all entity and component data
    }

    fn get_component_set<C: Component>(&mut self) -> Option<&mut SparseSet<C>> {
        let type_id = TypeId::of::<C>();
        if let Some(set) = self.items.get_mut(&type_id) {
            return set.as_any_mut().downcast_mut::<SparseSet<C>>();
        } else {
            None
        }
    }
}

fn main() {
    let mut entity_manager = EntityManager::new(); // Create Entity Manager 

    let mut world: World  = World::new(entity_manager); // Create world and attach the entity manager 
    
    world.ensure_set::<Position>(); // Ensures that world has a sparseset for position, and if it doesn't, creates one 

    
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