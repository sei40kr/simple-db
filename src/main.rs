use std::io::Write;

fn print_prompt() {
    print!("db > ");
    std::io::stdout().flush().unwrap();
}

fn main() {
    loop {
        print_prompt();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();

        if input == ".exit" {
            break;
        } else {
            println!("Unrecognized command '{}'.", input.trim());
        }
    }
}
