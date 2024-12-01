use std::env;
use std::fs::File;
use std::io::Read;

// TODO: Lotta unwrapping in this here entry point. Maybe look into that.

fn main() {
    let args: Vec<String> = env::args().collect();

    // TODO: Args validation?
    let day_str = args.get(1).unwrap();
    let day = day_str.parse::<i32>().unwrap();
    let challenge = args.get(2).unwrap();
    let input = load_input(day);

    println!("{}", input);

    match day {
        1 => todo!(),
        _ => panic!("No implementation for {}", day),
    }
}

fn load_input(day: i32) -> String {
    let path = format!("input/{}.input", day);
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
    //file.read
}
