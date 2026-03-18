use rand::seq::SliceRandom;
use rand::prelude::IndexedRandom;

pub fn generate_password() -> String {
    const SIZE: usize = 24;

    let upper: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let lower: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    let digits: &[u8] = b"0123456789";
    let special: &[u8] = b"!@#$%&*";

    let mut rng = rand::rng();
    let mut password = Vec::with_capacity(SIZE);

    // Ensure at least one of each category
    password.push(*upper.choose(&mut rng).unwrap() as char);
    password.push(*lower.choose(&mut rng).unwrap() as char);
    password.push(*digits.choose(&mut rng).unwrap() as char);
    password.push(*special.choose(&mut rng).unwrap() as char);

    let all = [upper, lower, digits, special].concat();

    for _ in password.len()..SIZE {
        password.push(*all.choose(&mut rng).unwrap() as char);
    }

    password.shuffle(&mut rng);

    password.into_iter().collect()
}