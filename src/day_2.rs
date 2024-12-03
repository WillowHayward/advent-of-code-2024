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
    for report_raw in reports {
        let report = parse_report(report_raw);
        let failures = check_report(report);

        if failures.len() == 0 {
            safe_count += 1;
        }
    }

    safe_count.to_string()
}

fn run_gold(input: String) -> String {
    let reports = input.lines();

    let mut safe_count = 0;
    for report_raw in reports {
        let report = parse_report(report_raw);
        let failures = check_report(report.to_vec());

        if failures.len() == 0 {
            safe_count += 1;
        } else if retry_report(report, failures) {
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

fn parse_report(input: &str) -> Vec<i32> {
    let values: Vec<i32> = input
        .split_whitespace()
        .map(|chunk| chunk.parse().unwrap())
        .collect();

    values
}

// Returns a Vec containing each position the dampener fired in
fn check_report(values: Vec<i32>) -> Vec<usize> {
    // NOTE: Assumes at least 2 values in report

    let mut dampened_indexes: Vec<usize> = Vec::new();
    let mut trend: Direction = Direction::Initial;

    let length = values.len() - 1;
    for i in 0..length {
        let direction = compare_values(values[i], values[i + 1]);

        if trend == Direction::Initial && direction != Direction::Invalid {
            trend = compare_values(values[i], values[i + 1]); // Sorry rust borrow checker, this
            // was easier than learning how to handle you today
        }


        if !check_result(&direction, &trend) {
            dampened_indexes.push(i);
        }

    }

    dampened_indexes
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

// All aboard the Brute Force Express
fn retry_report(values: Vec<i32>, dampened_indexes: Vec<usize>) -> bool {
    let length = values.len();
    for i in 0..length { // Choo chooooooo
        //println!("{}", i);
        //println!("{}", j);
        let mut values_copy = values.to_vec();
        values_copy.remove(i);
        //println!("Trying {:?}", values_copy);
        if check_report(values_copy).len() == 0 {
            return true;
        }
    }
    /*let num_failures = dampened_indexes.len();
    if num_failures > 2 {
        // If > 2 failures, definitively out of dampener limit
        //println!("Too many failures");
        return false;
    }
    // For 1-2 failures, could be within dampener limit - gotta check

    if num_failures == 2 {
        if dampened_indexes[1] - dampened_indexes[0] > 2 {
            // If not consecutive, two distinct failures
            //println!("Failures too distant");
            return false;
        }
    }

    //println!("Retrying {:?}", values);
    //println!("Dampened indexes: {:?}", dampened_indexes);
    // In theory I think you could just check around the dampened indexes, but that makes it tricky
    // with the trend so ¯\_(ツ)_/¯

    let mut indices_to_remove: Vec<usize> = dampened_indexes.to_vec();
    let length = dampened_indexes.len();
    for i in 0..length {
        indices_to_remove.push(dampened_indexes[i] + 1)
    }
    //indices_to_remove.push(indices_to_remove.last().unwrap() + 1);

    //println!("--, {:?}", indices_to_remove);
    for i in indices_to_remove {
        //println!("{}", i);
        //println!("{}", j);
        let mut values_copy = values.to_vec();
        values_copy.remove(i);
        //println!("Trying {:?}", values_copy);
        if check_report(values_copy).len() == 0 {
            return true;
        }
    }

    println!(
        //"Could not salvage {:?} - Errors {:?}",
        "{:?} {:?}",
        values, dampened_indexes
    );*/

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    /*#[test]
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
    }*/

    /*#[test]
    fn test_misc() {
        let test_a_input = parse_report("94 95 93 90 93");
        let test_a = check_report(test_a_input);
        assert_eq!(test_a, vec![1, 2,
    }*/
}
