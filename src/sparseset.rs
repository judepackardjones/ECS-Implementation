
use std::any::Any;

use crate::ecs::{components::*};
use crate::entity_storage::*;

pub struct SparseSet<C: Component> {
    pub dense: Vec<EntityId>, // May have [50, 2, 8, 4]
    pub components: Vec<C>, // [(x: 90, y: 70), (x: 2, (y: 5), (x: 10, y: 10), (x: 0, y: 35))]
    pub sparse: Vec<usize>, // Filled mostly with nothing. sparse[50] = 0, sparse[8] = 2
}

pub trait AnySparseSet: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

}

impl<C: Component> AnySparseSet for SparseSet<C> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}


impl<C: Component> SparseSet<C> {
    pub fn new() -> Self {
        SparseSet {
            dense: Vec::new(),
            components: Vec::new(),
            sparse: Vec::new()
        }
    }

    pub fn remove_component(&mut self, id: &EntityId) -> Result<(), &str> { // TODO
        let index = id.index as usize;
        if index > self.sparse.len() {
            return Err(&"Index out of range")
        }
        let dense_index = self.sparse[index];
        if id.generation != self.dense[dense_index].generation {
            return Err(&"Stale ID")
        }
        let last_index = self.dense.len() - 1;
        let last_component_id = self.dense[last_index];

        self.dense.swap(dense_index, last_index); // Possibly Handle Later
        self.components.swap(dense_index, last_index);

        self.sparse[last_component_id.index as usize] = dense_index;
        self.dense.pop();
        self.components.pop();
        Ok(())

    }

    pub fn add_component(&mut self, id: EntityId, component: C) -> Result<(), &str>{
        let index = id.index as usize;

        if index >= self.sparse.len() { //
            self.sparse.resize(index + 1, usize::MAX); //[USIZE::MAX]
        }

        let dense_index: usize = self.sparse[index]; // = USIZE::MAX 

        if let Some(val) = self.dense.get(dense_index) {
            if val.generation != id.generation {
                return Err(&"Stale ID")
            }
            return Err(&"Component already exists")
        }

        self.dense.push(id);
        self.components.push(component);
        self.sparse[index] = self.dense.len() - 1;
        
    

        Ok(())
       
    }
    pub fn get_mut_component(&mut self, id: &EntityId) -> Option<&mut C> {
        let index = id.index as usize;
        if index >= self.sparse.len() { // Catches if the entity has been added but no components so the sparse array isn't large enough yet 
            return None
        }
        let dense_index = self.sparse[index]; 

        if dense_index >= self.components.len() { // Catches if there is a entity ID added but no component yet 
            return None
        }
        let stored_entity_id = &self.dense[dense_index];
        if stored_entity_id.generation != id.generation { // Makes sure the previous dense storage generation matches the id's generation to make sure we are not referencing old data. 
            return None
        }

        Some(&mut self.components[dense_index])

    }
    pub fn read_component(&self, id: &EntityId) -> Option<&C> {
        let index = id.index as usize;
        if index >= self.sparse.len() { // Catches if the entity has been added but no components so the sparse array isn't large enough yet 
            return None
        }
        let dense_index = self.sparse[index]; 

        if dense_index >= self.components.len() { // Catches if there is a entity ID added but no component yet 
            return None
        }
        let stored_entity_id = &self.dense[dense_index];
        if stored_entity_id.generation != id.generation { // Makes sure the previous dense storage generation matches the id's generation to make sure we are not referencing old data. 
            return None
        }

        Some(&self.components[dense_index])

    }
}
