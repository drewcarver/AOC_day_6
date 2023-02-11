use std::fs;

fn is_unique(input: String) -> bool {
    let mut chars = input.chars().collect::<Vec<char>>();
    chars.sort();
    chars.dedup();

    let result = chars.len() == input.len();

    result
}

fn find_start_of_transmission(window_size: usize, input: String) -> Result<usize, String> {
    let transmission = input.chars().collect::<Vec<char>>();

    (window_size..transmission.len())
    .map(|i| (i-window_size, i))
    .take_while(|i| !is_unique(transmission[i.0..i.1].iter().collect::<String>()))
    .collect::<Vec<_>>()
    .last()
    .map(|indices| indices.1 + 1)
    .ok_or("Unable to find a non-duplicate".to_string())
}

fn main() {
    let is_problem_one = true;
    let window_size = if is_problem_one { 4 } else { 14 };
    let result = fs::read_to_string("input.txt")
        .map_err(|_| "Unable to read file".to_string())
        .and_then(|input| find_start_of_transmission(window_size, input));

    match result {
        Ok(r) => println!("Beginning is: {}", r),
        Err(e) => println!("{}", e)
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_first_four_non_duplicates() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();

        assert_eq!(Ok(7), find_start_of_transmission(4, input));
    }
}
