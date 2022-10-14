use figlet_rs::FIGfont;
use rand::Rng;
use std::fs;
extern crate colorful;
use std::{thread, time};

use colorful::{Colorful, HSL};

fn read_file_to_vector(path: &str) -> Vec<String> {
    return fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
}

fn main() {
    let mut rng = rand::thread_rng();
    let languages = read_file_to_vector("languages.txt");
    let num_langs = languages.len();

    let standard_font = FIGfont::standand().unwrap();
    let rand_lang = languages[rng.gen_range(0..num_langs)].clone();

    let one_second = time::Duration::from_secs(1);
    let rand_secs = rng.gen_range(3..60);
    for n in (1..=rand_secs).rev() {
        print!("{esc}c", esc = 27 as char);
        let fig_num = standard_font.convert(&format!("{n}")).unwrap();
        println!("{}", fig_num.to_string());
        thread::sleep(one_second);
    }

    print!("{esc}c", esc = 27 as char);
    let figure = standard_font.convert(&format!("{rand_lang}")).unwrap();
    let text = format!("{:^50}", figure.to_string());
    text.clone().rainbow_with_speed(4);

    let p: f32 = rng.gen();
    let h = (p * 1500.0 % 360.0) / 360.0;
    println!("{}", text.gradient(HSL::new(h, 1.0, 0.5)))
}
