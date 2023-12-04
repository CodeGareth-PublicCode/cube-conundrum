use std::cmp::max;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

pub fn parse_game_statement_to_max_number_hashmap(input: &str) -> HashMap<String, i16> {
    let split_results = input.split(": ").collect::<Vec<&str>>();

    let _game_id = split_results.get(0).unwrap().to_string();
    let cube_pulls = split_results.get(1).unwrap().to_string();

    let binding = cube_pulls.replace(";", ",");
    let individual_pulls: Vec<&str> = binding.split(", ").collect::<Vec<&str>>();

    let mut game_store: Vec<(String, i16)> = Vec::new();

    for individual in individual_pulls {

        let parsed_info: Vec<&str> = individual.split(" ").collect();
        game_store.push((String::from(parsed_info.get(1).unwrap().to_owned()), parsed_info.get(0).unwrap().parse().unwrap()))
    }

    let mut max_store: HashMap<String, i16> = HashMap::new();

    for pull in game_store {
        max_store.entry(pull.0).and_modify(|element| *element = max(*element, pull.1)).or_insert(pull.1);
    }

    max_store

}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap};

    #[test]
    fn parse_statement_to_max_hashmap() {

        let game_statement = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let max_hashmap = parse_game_statement_to_max_number_hashmap(&game_statement);

        assert_eq!(HashMap::from([(String::from("blue"), 6), (String::from("red"), 4), (String::from("green"), 2)]), max_hashmap)

    }

    #[test]
    fn compare_game_hashmap_to_rule_hashmap() {

        let game_statement = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let max_hashmap: HashMap<String, i16> = parse_game_statement_to_max_number_hashmap(&game_statement);

        let rule_input = [(String::from("blue"), 5), (String::from("red"), 4), (String::from("green"), 2)];
        let rule_hashmap: HashMap<String, i16> = HashMap::from(rule_input);

        let mut valid_game: bool = true;

        for (rule, max_allowed_value) in rule_hashmap.iter() {

            if max_hashmap.get_key_value(rule).unwrap().1 > max_allowed_value {
                valid_game = false;
                return
            }
            else {
            }

        assert_eq!(valid_game, false) // game rules state theres 5 blue but game log states 6 blue

        }

    }


}
