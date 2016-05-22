extern crate rand;
#[macro_use]
extern crate clap;

use rand::Rng;
use rand::os::OsRng;
use clap::App;

use std::str::FromStr;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let plength = u8::from_str(matches.value_of("plength").unwrap_or("3")).expect("NUM must be a number between 0 and 255");

    let words = include_str!("diceware8k.txt").lines().collect();
    let mut rng = OsRng::new().unwrap();

    for _ in 0..plength {
        print!("{} ", random_word(&mut rng, &words));
    }
}

fn random_word<T: Rng>(rng: &mut T, words: &Vec<&'static str>) -> &'static str {
    rng.choose(words).unwrap()
}
