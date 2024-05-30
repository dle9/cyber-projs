pub struct Player {
    pub level: u32,
    pub name: String,
    pub class: Class,
    pub skills: Skills,
    pub skill_points: u32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            level: 1,
            name: String::from("Guest"),
            class: Class::Serf,
            skills: Skills::new(&Class::Serf),
            skill_points: 3,
        }
    }

    pub fn get_skills(&self, class: Class) -> String {
        let skills = Skills::new(&class);
        format!(
            "Health: {}\nStrength: {}\nAgility: {}\nIntelligence: {}\nWisdom: {}",
            skills.health, skills.strength, skills.agility, skills.intelligence, skills.wisdom
        )
    }
}

pub struct Skills {
    health: u32,
    strength: u32,
    agility: u32,
    intelligence: u32,
    wisdom: u32,
}

impl Skills {
    fn new(class: &Class) -> Self {
        match class {
            Class::Serf => Skills {
                health: 2,
                strength: 6,
                agility: 3,
                intelligence: 1,
                wisdom: 3,
            },
            Class::Bibliosoph => Skills {
                health: 1,
                strength: 1,
                agility: 1,
                intelligence: 7,
                wisdom: 5,
            },
            Class::Vagabond => Skills {
                health: 3,
                strength: 3,
                agility: 3,
                intelligence: 3,
                wisdom: 3,
            },
            Class::Pariah => Skills {
                health: 2,
                strength: 3,
                agility: 6,
                intelligence: 2,
                wisdom: 2,
            },
        }
    }
}

pub enum Class {
    Serf,
    Bibliosoph,
    Vagabond,
    Pariah,
}