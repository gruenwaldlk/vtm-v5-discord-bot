use log::warn;
use serde::Deserialize;
use serde::Serialize;

const MAX_ATTRIBUTE: u8 = 5;
const MAX_SKILL: u8 = 5;
const MAX_DISCIPLINE: u8 = 5;

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct Character {
    name: String,
    concept: String,
    predator_type: String,
    chronicle: String,
    ambition: String,
    clan: String,
    sire: String,
    desire: String,
    generation: u8,
    attributes: std::collections::HashMap<String, Attribute>,
    skills: std::collections::HashMap<String, Skill>,
    disciplines: std::collections::HashMap<String, Discipline>,
    hunger: u8,
    humanity: u8,
    health: u8,
    health_damage: u8,
    aggravated_health_damage: u8,
    willpower: u8,
    willpower_damage: u8,
    aggravated_willpower_damage: u8,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct Attribute {
    id: String,
    text_id: String,
    points: u8,
}
impl Attribute {
    pub(crate) fn id(&self) -> &str {
        &self.id
    }
    pub(crate) fn text_id(&self) -> &str {
        &self.text_id
    }
    pub(crate) fn points(&self) -> u8 {
        if 0 <= self.points && self.points <= MAX_ATTRIBUTE {
            return self.points;
        } else {
            warn!(
                "The attribute \"{}\" has the unreasonable value of \"{}\".",
                self.id, self.points
            );
            return 0;
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct Skill {
    id: String,
    text_id: String,
    points: u8,
}
impl Skill {
    pub(crate) fn id(&self) -> &str {
        &self.id
    }
    pub(crate) fn text_id(&self) -> &str {
        &self.text_id
    }
    pub(crate) fn points(&self) -> u8 {
        if 0 <= self.points && self.points <= MAX_SKILL {
            return self.points;
        } else {
            warn!(
                "The skill \"{}\" has the unreasonable value of \"{}\".",
                self.id, self.points
            );
            return 0;
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct Discipline {
    id: String,
    text_id: String,
    points: u8,
    powers: std::collections::HashMap<String, Power>,
}
impl Discipline {
    pub(crate) fn id(&self) -> &str {
        &self.id
    }
    pub(crate) fn text_id(&self) -> &str {
        &self.text_id
    }
    pub(crate) fn points(&self) -> u8 {
        if 0 <= self.points && self.points <= MAX_DISCIPLINE {
            return self.points;
        } else {
            warn!(
                "The discipline \"{}\" has the unreasonable value of \"{}\".",
                self.id, self.points
            );
            return 0;
        }
    }
    pub(crate) fn power_by_id(&self, id: String) -> &Power {
        self.powers
            .get(&id)
            .expect(&format!("Unknown power with id \"{}\"", id))
    }
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct Power {
    id: String,
    text_id: String,
}
