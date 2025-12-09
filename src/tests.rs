mod tests {
    use crate::{ecs::components::Position, entity_storage::{EntityManager, World}};

    
    #[test]
    fn add_component() {
        let mut entity_manager = EntityManager::new();
        let mut world: World = World::new();
        
        let first_entity_id = entity_manager.get_fresh_id();
        let position = Position {x: 1, y: 20};
        world.ensure_set::<Position>();


        world.get_component_set::<Position>().unwrap().add_component(first_entity_id.clone(), position).unwrap();
        let position_value = world.get_component_set::<Position>().unwrap().get_mut_component(&first_entity_id).unwrap();
        assert_eq!(position_value, &Position {x: 1, y: 20});
    }
}