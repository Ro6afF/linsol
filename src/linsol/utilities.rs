extern crate rand;

#[allow(dead_code)]
pub fn get_random_name(len: usize) -> String {
    (0..len)
        .map(|_| (0x20u8 + (rand::random::<f32>() * 96.0) as u8) as char)
        .collect()
}
