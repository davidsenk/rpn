use std::io;
use std::io::Write;
use std::num::ParseFloatError;
use std::str::FromStr;

fn main() {
    println!("RPN Calculator");

    let mut stack: Vec<f64> = Vec::new();

    loop {
        let mut input = String::new();

        print!("> ");

        io::stdout()
            .flush()
            .expect("flush to stdout should not fail");

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let func = match input.trim() {
            "q" => Function::Quit,
            "+" => Function::Add,
            "-" => Function::Subtract,
            "*" | "x" => Function::Multiply,
            "/" => Function::Divide,
            "%" => Function::Modulo,
            "clear" | "c" => Function::ClearStack,
            _ => Function::Push(f64::from_str(input.trim())),
        };

        match func {
            Function::Push(x) => stack_push(&mut stack, x),
            Function::Quit => break,
            Function::ClearStack => stack.clear(),
            others => operate(&mut stack, others),
        }

        println!("Current Stack: ");

        for i in &stack {
            println!("{i}");
        }
    }
}

enum Function {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Push(Result<f64, ParseFloatError>),
    ClearStack,
    Quit,
}

fn stack_push(stack: &mut Vec<f64>, num: Result<f64, ParseFloatError>) {
    match num {
        Ok(num) => stack.push(num),
        Err(e) => println!("Cannot push:  {e}."),
    }
}

fn stack_too_small_err(func: &str) {
    println!("Stack must contain at least 2 numbers to {func}!");
}

fn operate(stack: &mut Vec<f64>, func: Function) {
    if stack.len() < 2 {
        match func {
            Function::Add => stack_too_small_err("add"),
            Function::Subtract => stack_too_small_err("subtract"),
            Function::Multiply => stack_too_small_err("multiply"),
            Function::Divide => stack_too_small_err("divide"),
            Function::Modulo => stack_too_small_err("modulo"),
            _ => {}
        }
        return;
    }

    let x = stack.pop().unwrap_or(0f64);
    let y = stack.pop().unwrap_or(0f64);

    match func {
        Function::Add => stack.push(x + y),
        Function::Subtract => stack.push(x - y),
        Function::Divide => stack.push(y / x),
        //x is at top of stack, but divide is order
        //dependent. We're dividing the number input prior
        //to x (y), by x. i.e. x: 3, y: 6, push = 0.5
        Function::Multiply => stack.push(x * y),
        Function::Modulo => stack.push(y % x),
        //same reason as division above
        _ => {} //do nothing, as we don't know what to do with it
    }
}
