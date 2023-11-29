use std::env;
use std::fs::File;
use rand::{self, Rng};
mod write_to_bmp;

fn main() {
    let args: Vec<String> = env::args().collect();
    let resolution: (u32, u32) = (args[1].parse::<u32>().unwrap(), args[2].parse::<u32>().unwrap());
    let save_dir = args[3].clone();
    let pixels: Vec<u16> = get_pixels(resolution.0 * resolution.1);
    let mut file = File::create(save_dir).unwrap();
    write_to_bmp::write_to_bmp(&mut file, pixels, &resolution);
}

fn get_pixels(n: u32) -> Vec<u16> {
    let mut res: Vec<u16> = vec![];
    for _ in 0..n + 1 {
        res.push(rand::thread_rng().gen_range(0..=65535));
    }
    res
}
