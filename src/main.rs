use std::io;
use std::io::Write;

fn main() {
    println!("Hello, world!");

    loop {
        let mut input = String::new();
        print!("> ");

        io::stdout()
            .flush()
            .expect("flush to stdout should not fail");

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let output = input.trim();

        println!("input received: '{output}'");
    }
}
