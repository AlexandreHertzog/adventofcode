use std::collections::HashSet;
use std::env;
use std::fs;
use std::process;

fn first_duplicate_frequency(filename: String) -> i32 {
    println!("Reading file: {}", filename);

    let mut sum = 0;
    let mut frequency_set: HashSet<i32> = HashSet::new();
    frequency_set.insert(sum);

    let mut found_duplicate = false;
    while !found_duplicate {
        let input_contents = fs::read_to_string(filename.clone()) 
            .expect("Error while reading input file");

        for line in input_contents.lines() {
            sum += line.parse::<i32>().unwrap();
            if frequency_set.contains(&sum) {
                found_duplicate = true;
                break;
            }
            
            frequency_set.insert(sum);
        }
    }
    sum
}

fn main() {
    println!("Beggining solution for Day 02 of advent of code");

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() <= 1 {
        println!("Program must have the input filename!");
        process::exit(1);
    }

    let result = first_duplicate_frequency(args[1].clone());
    println!("The first duplicate is: {}", result);
}

#[cfg(test)]
mod test {
    use crate::first_duplicate_frequency;

    #[test]
    fn first_duplicate_frequency_test() {
        assert_eq!(0, first_duplicate_frequency("input.test.01".to_string()));
        assert_eq!(10, first_duplicate_frequency("input.test.02".to_string()));
        assert_eq!(5, first_duplicate_frequency("input.test.03".to_string()));
        assert_eq!(14, first_duplicate_frequency("input.test.04".to_string()));
    }
}
