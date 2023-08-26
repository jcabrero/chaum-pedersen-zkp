use rand::Rng;

pub fn generate_random_string(length: usize) -> String {
    let charset: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    let random_string: String = (0..length)
        .map(|_| {
            let random_index = rng.gen_range(0..charset.len());
            charset[random_index] as char
        })
        .collect();

    random_string
}