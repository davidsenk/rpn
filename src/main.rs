use rpn::RPN;
use std::io;
use std::io::Write;

fn main() {
    println!("RPN Calculator (with FORTH)");

    let mut rpn = RPN::new();
    loop {
        let mut input = String::new();

        print!("> ");

        io::stdout()
            .flush()
            .expect("flush to stdout should not fail");

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        match rpn.eval(&input) {
            Err(e) => println!("Failed: {e:?}"),
            Ok(_) => {
                println!("Current Stack: ");
                for n in rpn.stack() {
                    println!("{n}");
                }
            }
        }
    }
}
