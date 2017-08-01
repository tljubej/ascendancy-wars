use rand::{thread_rng, Rng};
use rand::distributions::{IndependentSample, Weighted, WeightedChoice};
use std::fmt::{Display, Formatter};

const N_VOWELS: usize = 5;
const N_CONSONANTS: usize = 21;
const MIN_NAME_LEN: u32 = 1;
const MAX_MIN_NAME_LEN: u32 = 3;
const MAX_NAME_LEN: u32 = 7;
const MAX_TWO_CONSONANTS_CHANCE: f64 = 0.1;
const MAX_FLIP_ORDER_CHANCE: f64 = 0.3;
const MAX_TRAILING_CONSONANT_CHANCE: f64 = 0.6;

static VOWEL_DISTRIBUTION: [u32; N_VOWELS] = [33, 33, 20, 6, 4];
static CONSONANT_DISTRIBUTION: [u32; N_CONSONANTS] = [
    20,
    15,
    15,
    15,
    5,
    3,
    3,
    4,
    2,
    2,
    2,
    1,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];

static VOWELS: [char; N_VOWELS] = ['a', 'e', 'i', 'o', 'u'];
static CONSONANTS: [char; N_CONSONANTS] = [
    'b',
    'c',
    'd',
    'f',
    'g',
    'h',
    'j',
    'k',
    'l',
    'm',
    'n',
    'p',
    'q',
    'r',
    's',
    't',
    'v',
    'w',
    'x',
    'y',
    'z',
];


pub struct Culture {
    name: String,
    vowel_probabilities: Vec<Weighted<char>>,
    consonant_probabilities: Vec<Weighted<char>>,
    flip_order_chance: f64,
    two_consonants_chance: f64,
    trailing_consonant_chance: f64,
    min_name_length: u32,
    max_name_length: u32,
}

impl Display for Culture {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        fn print_probs(probs: &Vec<Weighted<char>>) -> Vec<String> {
            probs
                .iter()
                .map(|weighted| {
                    format!("({},{})", weighted.weight, weighted.item)
                })
                .collect()
        }

        write!(
            f,
            "{} {:?}, {:?}, foc:{}, tcc:{}, trailcc:{} minnl:{} maxnl:{}",
            self.name,
            print_probs(&self.vowel_probabilities),
            print_probs(&self.consonant_probabilities),
            self.flip_order_chance,
            self.two_consonants_chance,
            self.trailing_consonant_chance,
            self.min_name_length,
            self.max_name_length
        )
    }
}

impl Culture {
    pub fn new() -> Culture {
        let mut rng = thread_rng();
        let mut vowel_probabilities = VOWEL_DISTRIBUTION.clone();
        let mut consonant_probabilities = CONSONANT_DISTRIBUTION.clone();

        rng.shuffle(&mut vowel_probabilities);
        rng.shuffle(&mut consonant_probabilities);

        fn zip_into_weighted(probabilities: &[u32], chars: &[char]) -> Vec<Weighted<char>> {
            probabilities
                .iter()
                .zip(chars.iter())
                .map(|(weight, item)| {
                    Weighted {
                        weight: *weight,
                        item: *item,
                    }
                })
                .collect()
        }

        let vowel_probabilities = zip_into_weighted(&vowel_probabilities, &VOWELS);
        let consonant_probabilities = zip_into_weighted(&consonant_probabilities, &CONSONANTS);

        let min_name_length = rng.gen_range(MIN_NAME_LEN, MAX_MIN_NAME_LEN);
        let mut culture = Culture {
            name: String::from(""),
            vowel_probabilities,
            consonant_probabilities,
            flip_order_chance: rng.gen_range(0.0, MAX_FLIP_ORDER_CHANCE),
            two_consonants_chance: rng.gen_range(0.0, MAX_TWO_CONSONANTS_CHANCE),
            trailing_consonant_chance: rng.gen_range(0.0, MAX_TRAILING_CONSONANT_CHANCE),
            min_name_length: min_name_length,
            max_name_length: rng.gen_range(min_name_length, MAX_NAME_LEN),
        };

        culture.name = culture.generate_name();

        culture
    }

    pub fn generate_name(&self) -> String {
        let mut rng = thread_rng();
        let mut result = String::from("");

        let mut vowel_probabilities = self.vowel_probabilities.clone();
        let vowel_probabilities = WeightedChoice::new(&mut vowel_probabilities);

        let mut consonant_probabilities = self.consonant_probabilities.clone();
        let consonant_probabilities = WeightedChoice::new(&mut consonant_probabilities);

        for i in 0..rng.gen_range(self.min_name_length, self.max_name_length + 1) {
            let mut vowel = vowel_probabilities.ind_sample(&mut rng).to_string();
            let mut consonant = consonant_probabilities.ind_sample(&mut rng).to_string();

            if rng.gen::<f64>() < self.two_consonants_chance {
                vowel = consonant_probabilities.ind_sample(&mut rng).to_string();
            }

            if rng.gen::<f64>() < self.flip_order_chance {
                if i == 0 {
                    vowel = vowel.to_uppercase();
                }

                result += &vowel;
                result += &consonant;
            } else {
                if i == 0 {
                    consonant = consonant.to_uppercase();
                }

                result += &consonant;
                result += &vowel;
            }
        }

        if rng.gen::<f64>() < self.trailing_consonant_chance {
            result += &consonant_probabilities.ind_sample(&mut rng).to_string();
        }

        result
    }
}
