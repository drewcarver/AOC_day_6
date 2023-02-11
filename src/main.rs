use std::fs;

fn is_unique(input: String) -> bool {
    let mut chars = input.chars().collect::<Vec<char>>();
    chars.sort();
    chars.dedup();

    let result = chars.len() == input.len();
    println!("input is: {}", input); 
    println!("result is: {}", result); 

    result
}

fn find_start_of_transmission(input: String) -> Result<usize, String> {
    let transmission = input.chars().collect::<Vec<char>>();
    println!("{}", transmission.len());
    let mut end_range_iter = (4..transmission.len())
        .into_iter()
        .map(|i| {
            (i-4, i)
        })
        .take_while(|i| !is_unique(transmission[i.0..i.1].iter().collect::<String>()));


   Ok(end_range_iter.next().unwrap().1)
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .map_err(|_| "Unable to read file".to_string());

    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_first_four_non_duplicates() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();

        assert_eq!(Ok(7), find_start_of_transmission(input));
    }
}
