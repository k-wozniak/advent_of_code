use regex::Regex;

fn main() {
    // Read the file into a single string.
    let program_memory = std::fs::read_to_string("./src/input.txt").unwrap();

    println!(
        "Sum of products: {}",
        sum_of_valid_instruction(&program_memory)
    );
}

fn sum_of_valid_instruction(program_memory: &str) -> i32 {
    let re_full = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let re_nums = Regex::new(r"\d+,\d+").unwrap();

    re_full
        .find_iter(program_memory)
        .map(|m| re_nums.find(m.as_str()).unwrap())
        .map(|m| m.as_str().split(",").collect::<Vec<&str>>())
        .map(|v| (v[0], v[1]))
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .map(|(a, b)| a * b)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_valid_instruction() {
        let program_memory =
            r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let result = sum_of_valid_instruction(program_memory);

        assert_eq!(result, 161);
    }

    #[test]
    fn test_sum_of_valid_instruction_with_do_and_donts() {
        let program_memory =
            r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let result = sum_of_valid_instruction(program_memory);

        assert_eq!(result, 48);
    }
}
