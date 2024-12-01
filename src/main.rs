use std::env;
use std::fs::File;
use std::io::Read;

mod day_1;
// TODO: Lotta unwrapping in this here entry point. Maybe look into that.

fn main() {
    let args: Vec<String> = env::args().collect();

    // TODO: Args validation?
    let day_str = args.get(1).unwrap();
    let day = day_str.parse::<i32>().unwrap();
    let challenge = args.get(2).unwrap();
    let input = load_input(day);

    let solution = match day {
        1 => day_1::run(challenge, input),
        _ => panic!("No implementation for {}", day),
    };

    print!("{}", solution);
}

fn load_input(day: i32) -> String {
    let path = format!("input/{}.input", day);
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
}
