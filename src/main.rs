mod ecs;

use std::{any::TypeId, cell::RefCell, collections::HashMap};
use crate::ecs::{entities::*, components::*, systems::*};
use macroquad::prelude::*;

struct World {
    
}
pub struct EntityId {
    index: u32,
    generation: u32,
}

pub struct EntityManager {
    generations: Vec<u32>, // The generation[EntityId.index] = Next valid generation
    free_indices_queue: Vec<u32>, // Freed indices that can be used later. 
    next_index: u32, // If the queue is empty, get the next highest index.  
}

impl EntityManager {
    fn new() -> Self {
        EntityManager {
            generations: Vec::new(), 
            free_indices_queue: Vec::new(),
            next_index: 0,
        }
    }
    fn get_fresh_id(&mut self) -> EntityId {
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
    fn remove_id(&mut self, id: EntityId) {
        self.generations[id.index as usize] += 1;
        self.free_indices_queue.push(id.index);
    }
    pub fn is_alive(&self, id: &EntityId) -> bool {
        let index = id.index as usize;
        if index >= self.generations.len() {
            return false;
        }
        id.generation + 1 == self.generations[index]
}
}




pub struct SparseSet<C> {
    pub dense: Vec<EntityId>, // May have [50, 2, 8, 4]
    pub components: Vec<C>, // [(x: 90, y: 70), (x: 2, (y: 5), (x: 10, y: 10), (x: 0, y: 35))]
    pub sparse: Vec<usize>, // Filled mostly with nothing. sparse[50] = 0, sparse[8] = 2
}

impl<C> SparseSet<C> {
    fn add_component(&mut self, )
}

fn main() {
    let mut entity_manager = EntityManager::new();

}


// #[macroquad::main("GOL")]
// async fn main() {

//     loop {
//         dispatch();
//         next_frame().await
//     }
// }

fn dispatch() { // Should control the ordering of systems (Draw should be last)

    DrawSystem::run()
}