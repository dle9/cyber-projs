struct Skills {
    level: u32,
    health: u32,
    strength: u32,
    dexterity: u32,
    intelligence: u32,
}

impl Skills {
    fn new() -> Self {
        Self {
            level: 1,
            health: 3,
            strength: 3,
            dexterity: 3,
            intelligence: 3,
        }
    }
}

pub struct Player {
    pub name: String,
    pub class: String,
    pub skill_points: u32,
    pub skills: Skills,
}

impl Player {
    pub fn new() -> Self {
        Player {
            name: String::from("Guest"),
            class: String::from("Peasant"),
            skill_points: 3,
            skills: Skills::new(),
        }
    }
}