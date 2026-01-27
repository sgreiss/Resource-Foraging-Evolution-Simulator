pub struct Resource {
    resource_type: ResourceType,
}

impl Resource {
    pub fn new(resource_type: ResourceType) -> Self {
        Resource { resource_type }
    }
}

pub enum ResourceType {
    Food,
    FoodSource,
}