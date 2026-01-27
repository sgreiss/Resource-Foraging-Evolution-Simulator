#[derive(Clone, Copy, Debug)]
pub struct Resource {
    resource_type: ResourceType,
}

impl Resource {
    pub fn new(resource_type: ResourceType) -> Self {
        Resource { resource_type }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ResourceType {
    Food,
    FoodSource,
}