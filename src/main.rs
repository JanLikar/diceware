extern crate rand;

use rand::Rng;
use rand::os::OsRng;

fn main() {
    let plength = 4;
    let words = include_str!("sskj.txt").lines().collect();
    let mut rng = OsRng::new().unwrap();

    for _ in 0..plength {
        print!("{} ", random_word(&mut rng, &words));
    }
}

fn random_word<T: Rng>(rng: &mut T, words: &Vec<&'static str>) -> &'static str {
    rng.choose(words).unwrap()
}
