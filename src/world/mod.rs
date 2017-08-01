use std::collections::HashMap;
use generation::names::Culture;
use std::rc::Rc;
use std::fmt::{Display, Formatter};

type Id = u64;

const MAX_HEALTH: f64 = 100.0;
const INITIAL_POWER_LEVEL_MEAN : f64 = 1.0;
const INITIAL_POWER_LEVEL_DEVIATION : f64 = 0.2;

pub struct Person {
    pub id: Id,
    pub culture: Rc<Culture>,
    pub name: String,
    pub power_level: f64,
    pub health: f64,
    pub subordinates: Vec<Id>,
    pub leader: Option<Id>,
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(
            f,
            "id:{} name:'{}' power_level:{} health:{}",
            self.id,
            self.name,
            self.power_level,
            self.health
        )
    }
}

pub struct World {
    next_id: Id,
    pub people: HashMap<Id, Person>,
    pub people_by_power_level: Vec<Id>,
}

impl World {
    pub fn new(num_people: usize, num_cultures: usize) -> World {
        let mut world = World {
            next_id: 0,
            people: HashMap::with_capacity(num_people),
            people_by_power_level: Vec::with_capacity(num_people),
        };

        let mut culture = Rc::new(Culture::new());
        for i in 0..num_people {
            if i % (num_people / num_cultures) == 0 {
                culture = Rc::new(Culture::new());
                println!("{}", i);
            }

            world.add_new_person(
                culture.clone(),
                format!("{} {}", culture.generate_name(), culture.generate_name()),
                (INITIAL_POWER_LEVEL_MEAN - INITIAL_POWER_LEVEL_DEVIATION / 2.0) + (i as f64 / num_people as f64) * INITIAL_POWER_LEVEL_DEVIATION
            );
        }

        world
    }

    pub fn add_new_person(&mut self, culture: Rc<Culture>, name: String, power_level: f64) {
        let person = Person {
            id: self.next_id,
            culture: culture,
            name: name,
            power_level: power_level,
            health: MAX_HEALTH,
            subordinates: Vec::new(),
            leader: None,
        };

        self.next_id += 1;

        match self.people_by_power_level.binary_search_by(|other| {
            self.people
                .get(other)
                .unwrap()
                .power_level
                .partial_cmp(&person.power_level)
                .unwrap()
        }) {
            Ok(index) => self.people_by_power_level.insert(index, person.id),
            Err(index) => self.people_by_power_level.insert(index, person.id),
        }

        self.people.insert(person.id, person);
    }
}
