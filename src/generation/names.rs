use rand::{thread_rng, Rng};
use rand::distributions::{Weighted, WeightedChoice, IndependentSample};
use std::fmt::{Display, Formatter};

const N_VOWELS: usize = 5;
const N_CONSONANTS: usize = 21;

static VOWEL_DISTRIBUTION: [u32; N_VOWELS] = [33, 33, 20, 10, 4];
static CONSONANT_DISTRIBUTION: [u32; N_CONSONANTS] = [
    20,
    15,
    15,
    15,
    1,
    5,
    3,
    3,
    2,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
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
    vowel_probabilities: Vec<Weighted<char>>,
    consonant_probabilities: Vec<Weighted<char>>,
    flip_order_chance: f64,
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
            "{:?}, {:?}, foc:{}, mnl:{}",
            print_probs(&self.vowel_probabilities),
            print_probs(&self.consonant_probabilities),
            self.flip_order_chance,
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

        Culture {
            vowel_probabilities,
            consonant_probabilities,
            flip_order_chance: rng.gen(),
            max_name_length: rng.gen_range(1, 6),
        }

    }

    pub fn generate_name(&self) -> String {
        let mut rng = thread_rng();
        let mut result = "".to_string();

        let mut vowel_probabilities = self.vowel_probabilities.clone();
        let vowel_probabilities = WeightedChoice::new(&mut vowel_probabilities);

        let mut consonant_probabilities = self.consonant_probabilities.clone();
        let consonant_probabilities = WeightedChoice::new(&mut consonant_probabilities);

        for i in 0..rng.gen_range(1, self.max_name_length + 1) {
            let mut vowel = vowel_probabilities.ind_sample(&mut rng).to_string();
            let mut consonant = consonant_probabilities.ind_sample(&mut rng).to_string();

            if rng.gen::<f64>() < self.flip_order_chance {
                if i == 0 {
                    vowel = vowel.to_uppercase();
                }

                result = result + &vowel;
                result = result + &consonant;
            } else {
                if i == 0 {
                    consonant = consonant.to_uppercase();
                }

                result = result + &consonant;
                result = result + &vowel;
            }
        }

        result
    }
}
