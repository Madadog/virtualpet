pub struct VirtualPet {
    pub name: String,
    pub age: f32,
    pub max_age: f32,
    pub health: f32,
    pub happiness: f32,
    pub hunger: f32,
    pub sleep: f32,
}

pub struct Food {
    pub name: String,
    pub power: f32,
    pub health: f32,
    pub size: f32,
}

impl VirtualPet {
    pub fn new(name: String) -> Self {
        VirtualPet {
            name,
            age: 0.0,
            max_age: 3600.0,
            health: 5.0,
            happiness: 5.0,
            hunger: 5.0,
            sleep: 5.0,
        }
    }
    pub fn feed(&mut self, food: Food) {
        self.hunger -= food.power;
        self.health += food.health;
        // self.stomach_capacity -= food.size;
    }
}
