struct CubePull {
    num: i8,
    colour: String,
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::cmp::max;
    use super::*;
    use std::collections::{HashMap, HashSet};
    use std::iter::zip;

    #[test]
    fn test_can_match_tuple() {
        let input: (&str, &str) = ("a", "red");

        let result = match input {
            (a, "red") => "hit",
            (_, &_) => "miss",
        };

        assert_eq!("hit", result)
    }

    #[test]
    fn work_on_case() {

        struct Game{
            blue: i16,
            red: i16,
            green: i16,
        }

        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let split_results = input.split(": ").collect::<Vec<&str>>();

        let game_id = split_results.get(0).unwrap().to_string();
        let cube_pulls = split_results.get(1).unwrap().to_string();

        let binding = cube_pulls.replace(";", ",");
        let individual_pulls: Vec<&str> = binding.split(", ").collect::<Vec<&str>>();

        let mut game_store: Vec<(&str, i16)> = Vec::new();

        for individual in individual_pulls {

            let parsed_info: Vec<&str> = individual.split(" ").collect();
            game_store.push((parsed_info.get(1).unwrap(), parsed_info.get(0).unwrap().parse().unwrap()))
        }

        game_store.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        game_store.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap());

        let mut max_store: HashMap<&str, i16> = HashMap::new();

        for pull in game_store {
            max_store.entry(pull.0).and_modify(|element| *element = max(*element, pull.1)).or_insert(pull.1);;
        }


    }
}
