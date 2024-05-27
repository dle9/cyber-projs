struct Skills {
    health: u32,
    strength: u32,
    dexterity: u32,
    intelligence: u32,
}

impl Skills {
    fn new() -> Self {
        Self {
            health: 3,
            strength: 3,
            dexterity: 3,
            intelligence: 3,
        }
    }
}

pub struct Player {
    name: String,
    class: String,
    skill_points: u32,
    skills: Skills,
}

impl Player {
    pub fn new() -> Self {
        Player {
            name: String::new(),
            class: String::new(),
            skill_points: 0,
            skills: Skills::new(),
        }
    }

    // fn display(&self) {
    //     println!("Name: {}", self.name);
    //     println!("Class: {}", self.class);
    //     println!("Skill points: {}", self.skill_points);
    //     println!("Skills:");
    //     println!("Health: {}", self.skills.health);
    //     println!("Strength: {}", self.skills.strength);
    //     println!("Dexterity: {}", self.skills.dexterity);
    //     println!("Intelligence: {}", self.skills.intelligence);
    // }
}