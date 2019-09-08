use std::collections::HashMap;
use std::fs;

fn common_string(left: &String, right: &String) -> String {
    let ret = left.chars().any(|c| right.contains(c));
    for c in left.chars() {
        if right.
        let counter = controller.entry(c).or_insert(0);
        *counter += 1;
    }

    let mut found = false;
    for value in controller.values() {
        if *value == count {
            found = true;
        }
    }
    found
}

fn solution(filename: String) -> String {
    let mut twice_counter = 0;
    let mut thrice_counter = 0;

    let input_contents = fs::read_to_string(filename).expect("Failed to read input file");
    for line in input_contents.lines() {
        if has_n_equal_chars(&line.to_string(), 2) {
            twice_counter += 1;
        }
        if has_n_equal_chars(&line.to_string(), 3) {
            thrice_counter += 1;
        }
    }

    twice_counter * thrice_counter
}

fn main() {
    println!(
        "The solution to day 2 is {}",
        solution("test/data/input".to_string())
    );
}

#[cfg(test)]
mod test {
    use crate::solution;

    #[test]
    fn functional_test() {
        assert_eq!("fgij", solution("test/data/input.test.01".to_string()));
    }
}
