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
            "floor" | "fl" => Function::Floor,
            "ceil" | "cl" => Function::Ceil,
            "round" | "r" => Function::Round,
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

#[derive(Debug)]
enum Function {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Floor,
    Ceil,
    Round,
    Abs,
    Pow,
    Sqrt,
    Cbrt,
    LogN,
    Log,
    Sin,
    Asin,
    Sinh,
    Acos,
    Cos,
    Cosh,
    Tan,
    Atan,
    Tanh,
    Epsilon,
    Euler,
    Pi,
    Tau,
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

fn idk_man(func: Function) {
    println!("I don't know how to do {func:?}!")
}

fn small_stack_err(count_expected: i32, func: &str) {
    println!("Stack must contain at least {count_expected} numbers to {func}!");
}

fn stack_way_too_small_err(func: &str) {
    small_stack_err(1, func)
}

fn stack_too_small_err(func: &str) {
    small_stack_err(2, func);
}

fn operate_single(stack: &mut Vec<f64>, func: Function) {
    if stack.len() < 1 {
        stack_way_too_small_err("anything!!");
        return;
    }

    let x = stack.pop().unwrap_or(0f64);

    match func {
        Function::Round => stack.push(x.round()),
        others => idk_man(others),
    }
}

fn operate(stack: &mut Vec<f64>, func: Function) {
    if stack.len() <= 1 {
        match func {
            Function::Add => stack_too_small_err("add"),
            Function::Subtract => stack_too_small_err("subtract"),
            Function::Multiply => stack_too_small_err("multiply"),
            Function::Divide => stack_too_small_err("divide"),
            Function::Modulo => stack_too_small_err("modulo"),
            Function::Floor => stack_too_small_err("floor"),
            Function::Ceil => stack_too_small_err("ceil"),
            _ => {} //only above functions require
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
        //same reason as division above
        Function::Modulo => stack.push(y % x),
        Function::Floor => stack.push(floor(x, y)),
        Function::Ceil => stack.push(ceil(x, y)),
        others => idk_man(others),
    }
}

fn floor(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

fn ceil(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}
