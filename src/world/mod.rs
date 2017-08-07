use std::collections::{HashMap, HashSet};
use generation::names::Culture;
use std::rc::Rc;
use std::fmt::{Display, Formatter};

pub type Id = usize;

const MAX_HEALTH: f64 = 100.0;
const INITIAL_POWER_LEVEL_MEAN: f64 = 1.0;
const INITIAL_POWER_LEVEL_DEVIATION: f64 = 0.2;

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
    people_by_power_level: Vec<Id>,
    cultures: Vec<Rc<Culture>>,
}

impl World {
    pub fn new(num_people: usize, num_cultures: usize) -> World {
        let mut world = World {
            next_id: 0,
            people: HashMap::with_capacity(num_people),
            people_by_power_level: Vec::with_capacity(num_people),
            cultures: Vec::with_capacity(num_cultures),
        };

        let mut culture = Rc::new(Culture::new());
        for i in 0..num_people {
            if i % (num_people / num_cultures) == 0 {
                culture = Rc::new(Culture::new());
            }

            world.add_new_person(
                culture.clone(),
                format!("{} {}", culture.generate_name(), culture.generate_name()),
                (INITIAL_POWER_LEVEL_MEAN - INITIAL_POWER_LEVEL_DEVIATION / 2.0) +
                    (i as f64 / num_people as f64) * INITIAL_POWER_LEVEL_DEVIATION,
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

        let index = self.get_power_level_index(person.power_level);
        self.people_by_power_level.insert(index, person.id);

        self.people.insert(person.id, person);
    }

    pub fn update_power_level(&mut self, id: Id, new_power_level: f64) {
        let mut person = self.get_person_by_id_mut(id);

        person.power_level = new_power_level;
    }

    fn get_power_level_index(&self, power_level: f64) -> Id {
        match self.people_by_power_level.binary_search_by(|&other| {
            self.get_person_by_id(other)
                .power_level
                .partial_cmp(&power_level)
                .expect("Power level compare failed, this should not be happening!")
        }) {
            Ok(index) => index,
            Err(index) => index,
        }
    }

    pub fn remove_people(&mut self, people: HashSet<Id>) {

        for p in people {
            let person = self.people.remove(&p).expect("Person should be deletable");

            match person.leader {
                Some(id) => {
                    let mut leader = self.get_person_by_id_mut(id);
                    leader.subordinates.retain(|&sub| sub != person.id)
                }
                None => (),
            }

            for subordinate in person.subordinates {
                let mut subordinate = self.get_person_by_id_mut(subordinate);

                subordinate.leader = None;
            }
        }

        self.rebuild_power_level_lookup();
    }

    fn get_person_by_id(&self, id: Id) -> &Person {
        self.people
            .get(&id)
            .expect("Person with specified id not found!")
    }

    fn rebuild_power_level_lookup(&mut self) {
        self.people_by_power_level = self.people
            .values()
            .filter(|person| person.leader == None)
            .map(|person| person.id)
            .collect();

        self.people_by_power_level.sort();
    }

    fn get_power_level_with_subordinates(&self, id: Id) -> f64 {
        let person = self.get_person_by_id(id);
        person.power_level +
            person
                .subordinates
                .iter()
                .map(|&sub| self.get_power_level_with_subordinates(sub))
                .sum::<f64>()
    }

    fn get_person_by_id_mut(&mut self, id: Id) -> &mut Person {
        self.people
            .get_mut(&id)
            .expect("Person with specified id not found!")
    }

    pub fn get_power_level_range(&self, from: f64, to: f64) -> &[Id] {
        let index_from = self.get_power_level_index(from);
        let index_to = self.get_power_level_index(to);

        let index_from = if index_from < 0 { 0 } else { index_from };
        let index_to = if index_to >= self.people_by_power_level.len() {
            self.people_by_power_level.len() - 1
        } else {
            index_to
        };

        &self.people_by_power_level[index_from..index_to]
    }
}
