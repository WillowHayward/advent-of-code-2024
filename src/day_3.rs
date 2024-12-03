use regex::Regex;
pub fn run(challenge: &String, input: String) -> String {
    match challenge.as_str() {
        "silver" => run_silver(input),
        "gold" => run_gold(input),
        _ => panic!("Challenge must be silver or gold"), // TODO: This could probably be generalised
                                                         // for a future day
    }
}

fn run_silver(input: String) -> String {
    let muls = get_muls(input);

    let mut total = 0;

    for mul in muls {
        let a = mul.0;
        let b = mul.1;
        total += a * b;
    }

    total.to_string()
}

fn run_gold(input: String) -> String {
    let muls = get_muls_with_commands(input);

    let mut total = 0;

    for mul in muls {
        let a = mul.0;
        let b = mul.1;
        total += a * b;
    }

    total.to_string()

}

fn get_muls(input: String) -> Vec<(i32, i32)> {
    let reg = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut muls: Vec<(i32, i32)> = Vec::new();

    for (_, [a, b]) in reg.captures_iter(&input).map(|c| c.extract()) {
        let a_parsed = a.to_string().parse().unwrap();
        let b_parsed = b.to_string().parse().unwrap();
        muls.push((a_parsed, b_parsed));
    }

    muls
}

fn get_muls_with_commands(input: String) -> Vec<(i32, i32)> {
    let mut muls: Vec<(i32, i32)> = Vec::new();

    let blocks = input.split("do()");

    for block in blocks {
        let mut subblocks = block.split("don't()");

        let mut submuls = get_muls(subblocks.next().unwrap().to_string());

        muls.append(&mut submuls);
    }

    muls
}
