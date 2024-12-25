use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let s = std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(length)
        .collect::<Vec<u8>>();
    String::from_utf8(s).unwrap()
}

pub fn rand_u64() -> u64 {
    rand::random()
}
