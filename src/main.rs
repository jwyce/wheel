use figlet_rs::FIGfont;
use rand::Rng;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;

fn read_file_to_vector(path: &str) -> Vec<String> {
    return fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
}

fn main() {
    let languages = read_file_to_vector("languages.txt");
    let people = read_file_to_vector("people.txt");

    let args: Vec<String> = env::args().collect();
    let config = &args[1];

    let mut results: HashMap<String, String> = HashMap::new();
    match config as &str {
        "all" => {
            for person in &people {
                let num_langs = languages.len();
                results.insert(
                    person.clone(),
                    languages[rand::thread_rng().gen_range(0..num_langs)].clone(),
                );
            }
        }
        _ => {
            let num_langs = languages.len();
            let person = people
                .iter()
                .find(|&x| *x == config.to_string())
                .expect("person not found");
            results.insert(
                person.clone(),
                languages[rand::thread_rng().gen_range(0..num_langs)].clone(),
            );
        }
    }

    let standard_font = FIGfont::standand().unwrap();
    for (person, language) in &results {
        let figure = standard_font
            .convert(&format!("{person}:   {language}"))
            .unwrap();
        write!(&mut rainbowcoat::stdout(), "{}", figure).unwrap();
    }
}
