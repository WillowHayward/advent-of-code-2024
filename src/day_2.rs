
pub fn run(challenge: &String, input: String) -> String {
    match challenge.as_str() {
        "silver" => run_silver(input),
        "gold" => run_gold(input),
        _ => panic!("Challenge must be silver or gold"), 
    }
}

fn run_silver(input: String) -> String {
    let reports = input.lines();

    let mut safe_count = 0;
    for report in reports {
        let safe = parse_report(report);

        if safe {
            safe_count += 1;
        }
    }

    safe_count.to_string()
}

#[derive(PartialEq, Eq)]
enum Direction {
    Invalid,
    Positive,
    Negative,
}

fn parse_report(input: &str) -> bool {
    let values: Vec<i32> = input
        .split_whitespace()
        .map(|chunk| chunk.parse().unwrap())
        .collect();

    // NOTE: Assumes at least 2 values in report

    let direction: Direction = compare_values(values[0], values[1]);

    if direction == Direction::Invalid {
        return false
    }

    let length = values.len() - 1;
    for i in 1..length {
        let new_direction = compare_values(values[i], values[i + 1]);

        if direction != new_direction {
            return false;
        }
    }

    true
}

fn compare_values(a: i32, b: i32) -> Direction {
    let diff = a - b;
    if diff.abs() > 3 || diff == 0 {
        return Direction::Invalid;
    }

    if a < b {
        return Direction::Negative;
    }

    if a > b {
        return Direction::Positive;
    }

    return Direction::Invalid;
}

fn run_gold(input: String) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_report() {
        let test_up_safe = parse_report("1 3 6 7 9");
        assert_eq!(test_up_safe, true);
        let test_down_safe = parse_report("7 6 4 2 1");
        assert_eq!(test_down_safe, true);
        let test_up_unsafe = parse_report("1 2 7 8 9");
        assert_eq!(test_up_unsafe, false);
        let test_down_unsafe = parse_report("9 7 6 2 1");
        assert_eq!(test_down_unsafe, false);
        let test_mixed_unsafe = parse_report("1 3 2 4 5");
        assert_eq!(test_mixed_unsafe, false);
        let test_even_unsafe = parse_report("8 6 4 4 1");
        assert_eq!(test_even_unsafe, false);
    }
}
