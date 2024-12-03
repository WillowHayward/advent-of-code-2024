pub fn run(challenge: &String, input: String) -> String {
    match challenge.as_str() {
        "silver" => run_silver(input),
        "gold" => run_gold(input),
        _ => panic!("Challenge must be silver or gold"),
    }
}

fn run_silver(input: String) -> String {
    run_report_check(input, false)
}

fn run_gold(input: String) -> String {
    run_report_check(input, true)
}

fn run_report_check(input: String, allow_dampener: bool) -> String {
    let reports = input.lines();

    let mut safe_count = 0;
    for report in reports {
        let safe = parse_report(report, allow_dampener);

        if safe {
            safe_count += 1;
        } 
    }

    safe_count.to_string()

}

#[derive(PartialEq, Eq)]
enum Direction {
    Initial,
    Invalid,
    Positive,
    Negative,
}

fn parse_report(input: &str, allow_dampener: bool) -> bool {
    let values: Vec<i32> = input
        .split_whitespace()
        .map(|chunk| chunk.parse().unwrap())
        .collect();

    // NOTE: Assumes at least 2 values in report

    let mut dampener_used = !allow_dampener;
    let mut dampened_index: i32 = -1;

    let mut trend: Direction = Direction::Initial;


    let length = values.len() - 1;
    for i in 0..length {
        if i as i32 == dampened_index {
            continue;
        }

        let direction = compare_values(values[i], values[i + 1]);

        if !check_result(&direction, &trend) {
            if dampener_used {
                println!("{}", input);
                println!("Dampener already used!");
                return false;
            }
            dampener_used = true;

            // Test removing left value
            let case_a = if i > 0 {
                compare_values(values[i - 1], values[i + 1])
            } else {
                // If it's not just the first digit that's an error, the dampener will fire again
                // in the next loop
                continue;
            };

            if check_result(&case_a, &trend) {
                trend = case_a;
                dampened_index = i.try_into().unwrap();
                //println!("{}", input);
                //println!("Passed without {}", values[i]);
                continue;
            }
            //println!("Still failed without {}", values[i]);

            let case_b = if i <= length - 2 {
                compare_values(values[i], values[i + 2])
            } else {
                // If it's reached the final digit without the dampener firing, it'll be correct
                return true;
            };

            if check_result(&case_b, &trend) {
                trend = case_b;
                dampened_index = (i + 1).try_into().unwrap();
                //println!("{}", input);
                //println!("Passed without {}", values[i + 1]);
                continue;
            }
            println!("{}", input);
            println!("Still failed without {}", values[i]);
            println!("Still failed without {}", values[i+1]);

            return false;

        }

        trend = direction;
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

fn check_result(new_direction: &Direction, old_direction: &Direction) -> bool {
    if *new_direction == Direction::Invalid {
        return false;
    }

    if *old_direction == Direction::Initial {
        return true;
    }

    if *new_direction != *old_direction {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_report_silver() {
        let test_a = parse_report("7 6 4 2 1", false);
        assert_eq!(test_a, true);
        let test_b = parse_report("1 2 7 8 9", false);
        assert_eq!(test_b, false);
        let test_c = parse_report("9 7 6 2 1", false);
        assert_eq!(test_c, false);
        let test_d = parse_report("1 3 2 4 5", false);
        assert_eq!(test_d, false);
        let test_e = parse_report("8 6 4 4 1", false);
        assert_eq!(test_e, false);
        let test_f = parse_report("1 3 6 7 9", false);
        assert_eq!(test_f, true);
    }

    #[test]
    fn test_parse_report_gold() {
        let test_a = parse_report("7 6 4 2 1", true);
        assert_eq!(test_a, true);
        let test_b = parse_report("1 2 7 8 9", true);
        assert_eq!(test_b, false);
        let test_c = parse_report("9 7 6 2 1", true);
        assert_eq!(test_c, false);
        let test_d = parse_report("1 3 2 4 5", true);
        assert_eq!(test_d, true);
        let test_e = parse_report("8 6 4 4 1", true);
        assert_eq!(test_e, true);
        let test_f = parse_report("1 3 6 7 9", true);
        assert_eq!(test_f, true);


        // Error at first position
        let test_g = parse_report("1 1 3 6 7 9", true);
        assert_eq!(test_g, true);

        // Error at last position
        let test_h = parse_report("1 3 6 7 9 9", true);
        assert_eq!(test_h, true);
    }
}
