use std::cmp::max;
use std::collections::HashMap;

// ---- PART 2 SECTION [START] ---

fn main() {
    let file_path: &str = "./src/input.txt";
    let content: String = std::fs::read_to_string(file_path).expect("should read from file");
    let content_vector = content.lines().collect::<Vec<&str>>();

    println!(
        "{}",
        total_sum_of_max_element_cubes_from_games(content_vector)
    );
}

pub fn cube_of_max_elements_from_game_statement(game_statement: &str) -> i32 {
    let max_game_hashmap: HashMap<String, i32> =
        parse_game_statement_to_max_number_hashmap(game_statement);

    max_game_hashmap.values().product()
}

// ---- PART 1 SECTION [START] ---

fn old_main() {
    let rule_hashmap = generate_rule_hashmap(14, 12, 13);

    let file_path: &str = "./src/input.txt";
    let content: String = std::fs::read_to_string(file_path).expect("should read from file");
    let content_vector = content.lines().collect::<Vec<&str>>();

    println!(
        "{}",
        sum_of_game_numbers_that_were_possible(&rule_hashmap, &content_vector)
    );
}

pub fn sum_of_game_numbers_that_were_possible(
    rule_hashmap: &HashMap<String, i32>,
    content: &Vec<&str>,
) -> usize {
    content
        .iter()
        .enumerate()
        .map(|(position, &game_statement)| {
            if !validate_if_game_was_impossible(&game_statement, &rule_hashmap) {
                position + 1
            } else {
                0
            }
        })
        .sum()
}

fn validate_if_game_was_impossible(
    game_statement: &&str,
    rule_hashmap: &HashMap<String, i32>,
) -> bool {
    let game_max_hashmap: HashMap<String, i32> =
        parse_game_statement_to_max_number_hashmap(&game_statement);

    // is any tuple where top number in game exceeds allowed number for same colour in rules,
    rule_hashmap.iter().any(|(rule, max_allowed_value)| {
        game_max_hashmap.get_key_value(rule).unwrap().1 > max_allowed_value
    })
}

pub fn parse_game_statement_to_max_number_hashmap(input: &str) -> HashMap<String, i32> {
    let split_results = input.split(": ").collect::<Vec<&str>>();

    let _game_id = split_results.get(0).unwrap().to_string();
    let cube_pulls = split_results.get(1).unwrap().to_string();

    let binding = cube_pulls.replace(";", ",");
    let individual_pulls: Vec<&str> = binding.split(", ").collect::<Vec<&str>>();

    let mut game_store: Vec<(String, i32)> = Vec::new();

    for individual in individual_pulls {
        let parsed_info: Vec<&str> = individual.split(" ").collect();
        game_store.push((
            String::from(parsed_info.get(1).unwrap().to_owned()),
            parsed_info.get(0).unwrap().parse().unwrap(),
        ))
    }

    let mut max_store: HashMap<String, i32> = HashMap::new();

    for pull in game_store {
        max_store
            .entry(pull.0)
            .and_modify(|element| *element = max(*element, pull.1))
            .or_insert(pull.1);
    }

    max_store
}

pub fn generate_rule_hashmap(blue: i32, red: i32, green: i32) -> HashMap<String, i32> {
    let rule_input = [
        (String::from("blue"), blue),
        (String::from("red"), red),
        (String::from("green"), green),
    ];
    let rule_hashmap: HashMap<String, i32> = HashMap::from(rule_input);
    rule_hashmap
}

pub fn total_sum_of_max_element_cubes_from_games(game_statements: Vec<&str>) -> i32 {
    let total_cube_of_max_elements_from_games: i32 = game_statements
        .iter()
        .map(|game_statement| cube_of_max_elements_from_game_statement(game_statement))
        .sum();
    total_cube_of_max_elements_from_games
}

// ---- PART 1 SECTION [END] ---

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_part_2_multiply_max_hash_from_game_statement() {
        let game_statement = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        assert_eq!(cube_of_max_elements_from_game_statement(game_statement), 12);
    }

    #[test]
    fn test_part_2_build_out_from_example_scenarios() {
        let game_statements: Vec<&str> = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        let total_cube_of_max_elements_from_games =
            total_sum_of_max_element_cubes_from_games(game_statements);

        assert_eq!(total_cube_of_max_elements_from_games, 2286);
    }

    #[test]
    fn test_part_1_parse_statement_to_max_hashmap() {
        let game_statement = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let max_hashmap = parse_game_statement_to_max_number_hashmap(&game_statement);

        assert_eq!(
            HashMap::from([
                (String::from("blue"), 6),
                (String::from("red"), 4),
                (String::from("green"), 2)
            ]),
            max_hashmap
        )
    }

    #[test]
    fn test_part_1_compare_game_hashmap_to_rule_hashmap() {
        let game_statement = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        // you're only allowed 6 blue, 1 red, 1 green
        let rule_hashmap = generate_rule_hashmap(6, 1, 1);

        let game_max_hashmap: HashMap<String, i32> =
            parse_game_statement_to_max_number_hashmap(&game_statement);

        // is any tuple where top number in game exceeds allowed number for same colour in rules,
        let invalid_game: bool = rule_hashmap.iter().any(|(rule, max_allowed_value)| {
            game_max_hashmap.get_key_value(rule).unwrap().1 > max_allowed_value
        });

        // Anticipate failure due to only allowing 1 green and 2 being present in game statement
        assert_eq!(invalid_game, true) // game rules state theres 5 blue but game log states 6 blue
    }

    #[test]
    fn test_part_1_find_impossible_game() {
        let rule_hashmap = generate_rule_hashmap(6, 4, 1);
        let game_statement = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let is_game_impossible = validate_if_game_was_impossible(&game_statement, &rule_hashmap);
        assert_eq!(is_game_impossible, true);
    }

    #[test]
    fn test_part_1_find_possible_game() {
        let rule_hashmap = generate_rule_hashmap(6, 4, 2);
        let game_statement = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let is_game_impossible = validate_if_game_was_impossible(&game_statement, &rule_hashmap);
        assert_eq!(is_game_impossible, false);
    }

    #[test]
    fn test_part_1_recreate_example() {
        let rule_hashmap = generate_rule_hashmap(14, 12, 13);

        let game_statements: Vec<&str> = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        let sum_of_game_numbers_that_were_possible =
            sum_of_game_numbers_that_were_possible(&rule_hashmap, &game_statements);

        assert_eq!(sum_of_game_numbers_that_were_possible, 8); // game 1,2,5 should be possible
    }
}
