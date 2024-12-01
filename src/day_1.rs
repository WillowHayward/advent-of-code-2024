use std::collections::HashMap;

pub fn run(challenge: &String, input: String) -> String {
    match challenge.as_str() {
        "silver" => run_silver(input),
        "gold" => run_gold(input),
        _ => panic!("Challenge must be silver or gold"), // TODO: This could probably be generalised
                                                         // for a future day
    }
}

fn run_silver(input: String) -> String {
    let lists = parse_lists(input);
    let solution = compare_lists(lists.0, lists.1);

    solution.to_string()
}

fn parse_lists(input: String) -> (Vec<i32>, Vec<i32>) {
    let lines = input.lines();

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    for line in lines {
        let items = parse_line(line);
        left_list.push(items.0);
        right_list.push(items.1);
    }

    // TODO: It might be faster to sort these are they're added. I suspect for part 2, it will be.
    left_list.sort();
    right_list.sort();

    (left_list, right_list)
}

fn parse_line(line: &str) -> (i32, i32) {
    let chunks: Vec<i32> = line
        .split_whitespace()
        .map(|chunk| chunk.parse().unwrap())
        .collect();

    (chunks[0], chunks[1])
}

// Assumes lists are sorted and of equal length
fn compare_lists(left_list: Vec<i32>, right_list: Vec<i32>) -> i32 {
    let length = left_list.len();
    let mut total_distance: i32 = 0;

    for i in 0..length {
        let left = left_list[i];
        let right = right_list[i];

        let difference = left - right;
        total_distance += difference.abs();
    }

    total_distance
}

fn run_gold(input: String) -> String {
    let lists = parse_lists(input);

    let left_duplicates = count_duplicates(lists.0);
    let right_duplicates = count_duplicates(lists.1);

    let score = calculate_similarity_score(left_duplicates, right_duplicates);

    score.to_string()
}

fn count_duplicates(list: Vec<i32>) -> HashMap<i32, i32> {
    let mut duplicates: HashMap<i32, i32> = HashMap::new();

    for item in list {
        let current = duplicates.get(&item);
        let total = match current {
            Some(i) => i + 1,
            None => 1,
        };
        duplicates.insert(item, total);
    }

    duplicates
}

fn calculate_similarity_score(left_duplicates: HashMap<i32, i32>, right_duplicates: HashMap<i32, i32>) -> i32 {
    let mut score = 0;

    for i in left_duplicates.keys() {
        let left_value = left_duplicates.get(&i).unwrap();
        let right_value: i32 = match right_duplicates.get(&i) {
            Some(v) => *v,
            None => 0
        };

        score += i * left_value * right_value;
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let test_1 = parse_line("0     6");
        assert_eq!(test_1, (0, 6));
        let test_2 = parse_line("  4     2  ");
        assert_eq!(test_2, (4, 2));
    }

    #[test]
    fn test_parse_list() {
        let input_1 = "\
1    8
3    1
2    4"
            .to_string();
        let test_1 = parse_lists(input_1);
        assert_eq!(test_1.0, vec![1, 2, 3]);
        assert_eq!(test_1.1, vec![1, 4, 8]);
    }

    #[test]
    fn test_count_duplicates() {
        let input_1 = vec![1, 2, 3, 3, 3, 1];
        let test_1 = count_duplicates(input_1);
        let expected_1 = HashMap::from([
            (1, 2),
            (2, 1),
            (3, 3)
        ]);
        assert_eq!(test_1, expected_1);
    }
}
