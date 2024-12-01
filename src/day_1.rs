pub fn run(challenge: &String, input: String) -> String {
    match challenge.as_str() {
        "silver" => run_silver(input),
        "gold" => run_gold(input),
        _ => panic!("Challenge must be silver or gold") // TODO: This could probably be generalised
        // for a future day
    }
}

fn run_silver(input: String) -> String {
    "Day 1 Silver".to_string()
}

fn run_gold(input: String) -> String {
    todo!()
}
