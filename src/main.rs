use chars::*;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() != 2 {
        print_usage(&argv[0]);
        return;
    }

    let input = &argv[1];

    let no_duplicated_input = remove_duplicated_chars(input);
    for c in no_duplicated_input.chars() {
        println!("{} = {}", c, c as u32);
    }
}

fn print_usage(app_name: &str) {
    println!("Usage: {} <input>", app_name);
}
