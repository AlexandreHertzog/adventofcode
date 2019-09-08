use std::env;
use std::fs;
use std::process;

fn sum_file_lines(filename: String) -> i32 {
    println!("Reading file: {}", filename);

    let input_contents = fs::read_to_string(filename) 
        .expect("Error while reading input file");

    let mut sum = 0;
    for line in input_contents.lines() {
        sum += line.parse::<i32>().unwrap();
    }
    sum
}

fn main() {
    println!("Beggining solution for Day 01 of advent of code");

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() <= 1 {
        println!("Program must have the input filename!");
        process::exit(1);
    }

    let result = sum_file_lines(args[1].clone());
    println!("The sum is: {}", result);
}

#[cfg(test)]
mod test {
    use crate::sum_file_lines;

    #[test]
    fn functional_test() {
        assert_eq!(9, sum_file_lines("input.test.01".to_string()));
    }
}
