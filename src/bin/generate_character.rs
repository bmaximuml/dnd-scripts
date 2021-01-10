extern crate rand;
extern crate yaml_rust;

use rand::seq::SliceRandom;
use std::io::Read;
use yaml_rust::{YamlLoader, Yaml};
use rand::Rng;

#[derive(Debug)]
struct Character<'a> {
    race: &'a String,
    class: &'a String,
    background: &'a String,
    attributes_raw: &'a Vec<i32>,
    attributes_chosen: &'a String
}

fn get_vec_from_yaml(y: &Yaml) -> Vec<String> {
    // Convert a Yaml Array into a Vec of Strings
    let mut vec: Vec<String> = Vec::new();
    for i in y.as_vec().unwrap().iter() {
        vec.push(i.as_str().unwrap().to_string());
    }
    vec
}

fn roll_attribute() -> i32 {
    // Roll four six-sided dice, return the sum of the highest three
    let mut rng = rand::thread_rng();

    let mut vec: Vec<i32> = Vec::new();
    for _ in 0..4 {
        vec.push(rng.gen_range(1..6));
    }

    vec.sort();
    vec.remove(0);
    vec.iter().sum()
}

fn choose_attributes(attributes: &Vec<i32>) -> String {
    // Take in 6 numbers and format them into D&D attributes
    String::from(format!(
        "INT: {}, CHA: {}, CON: {}, DEX: {}, STR: {}, WIS: {}",
        attributes[0],
        attributes[1],
        attributes[2],
        attributes[3],
        attributes[4],
        attributes[5]
    ))
}

fn main() {
    // Load in yaml file with character info
    let mut file = std::fs::File::open("src/bin/characters.yml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let docs = YamlLoader::load_from_str(&contents).unwrap();
    let docs = &docs[0];

    // Extract data from file
    let races: Vec<String> = get_vec_from_yaml(&docs["races"]);
    let classes: Vec<String> = get_vec_from_yaml(&docs["classes"]);
    let backgrounds: Vec<String> = get_vec_from_yaml(&docs["backgrounds"]);

    // Choose data
    let mut rng  = rand::thread_rng();
    let race: &String = races.choose(&mut rng).unwrap();
    let class: &String = classes.choose(&mut rng).unwrap();
    let background: &String = backgrounds.choose(&mut rng).unwrap();

    // Roll Attributes
    let mut attributes_raw: Vec<i32> = Vec::new();
    for _ in 0..6 {
        attributes_raw.push(roll_attribute());
    }
    let attributes_chosen = choose_attributes(&attributes_raw);
    attributes_raw.sort();
    attributes_raw.reverse();

    let character = Character {
        race: &race,
        class: &class,
        background: &background,
        attributes_raw: &attributes_raw,
        attributes_chosen: &attributes_chosen
    };

    // Print data
    println!("Race: {}", character.race);
    println!("Class: {}", character.class);
    println!("Background: {}", character.background);
    println!("Attributes (chosen): {:?}", character.attributes_chosen);
    println!("Attributes (all): {:?}", character.attributes_raw);
}