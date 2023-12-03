fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_match_tuple() {
        let input: (&str, &str) = ("a", "red");

        let result = match input {
            (a, "red") => { "hit" },
            (_, &_) => { "miss" }
        };

        assert_eq!("hit", result)
    }



}
