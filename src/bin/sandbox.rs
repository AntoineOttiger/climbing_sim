use rand::Rng;

fn get_rand_bool() -> bool {
    let mut rng = rand::thread_rng();
    rng.r#gen()
}

fn main() {
    let b = get_rand_bool();
    print!("{}", b);
}