fn parse_calories(input: &[&str]) -> Vec<u32> {
    let mut calories = vec![];
    let mut current_calories = 0;

    for line in input {
        if line.len() > 0 {
            // calories
            current_calories += line.parse::<u32>().unwrap();
        } else {
            // end of calorie list
            calories.push(current_calories);
            current_calories = 0;
        }
    }

    // the file reader cuts trailing newlines
    // so there can never be 0 current calories
    calories.push(current_calories);
    calories
}

pub fn solve_1(input: &[&str]) -> u32 {
    parse_calories(input).into_iter()
        .max().unwrap()
}

pub fn solve_2(input: &[&str]) -> u32 {
    let mut calories = parse_calories(input);
    calories.sort_unstable();
    calories.reverse();

    calories.into_iter()
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_2_example() {
        let input = ["1000", "2000", "3000", "",
            "4000", "",
            "5000", "6000", "",
            "7000", "8000", "9000", "",
            "10000"];

        assert_eq!(solve_2(&input), 45000);
    }
}