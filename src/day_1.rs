pub fn run(challenge: &String, input: String) -> String {
    match challenge.as_str() {
        "silver" => run_silver(input),
        "gold" => run_gold(input),
        _ => panic!("Challenge must be silver or gold") // TODO: This could probably be generalised
        // for a future day
    }
}

fn run_silver(input: String) -> String {
    let lists = parse_lists(input);
    "tada".to_string()
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
    let chunks: Vec<i32> = line.split_whitespace().map(|chunk| chunk.parse().unwrap()).collect();

    (chunks[0], chunks[1])
}

fn run_gold(input: String) -> String {
    todo!()
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
2    4".to_string();
        let test_1 = parse_lists(input_1);
        assert_eq!(test_1.0, vec![1, 2, 3]);
        assert_eq!(test_1.1, vec![1, 4, 8]);
    }
}
