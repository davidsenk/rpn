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
            "abs" => Function::Abs,
            "pow" | "^" => Function::Pow,
            "sqrt" | "v" => Function::Sqrt,
            "cbrt" | "v3" => Function::Cbrt,
            "ln" => Function::LogN,
            "log" => Function::Log,
            "sin" => Function::Sin,
            "asin" => Function::Asin,
            "sinh" => Function::Sinh,
            "acos" => Function::Acos,
            "cos" => Function::Cos,
            "cosh" => Function::Cosh,
            "tan" => Function::Tan,
            "atan" => Function::Atan,
            "tanh" => Function::Tanh,
            "me" => Function::Epsilon,
            "euler" | "e" => Function::Euler,
            "pi" => Function::Pi,
            "tau" => Function::Tau,
            "del" | "d" => Function::DeleteLast,
            "help" | "?" => Function::Help,
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
    DeleteLast,
    ClearStack,
    Help,
    Quit,
}

fn stack_push(stack: &mut Vec<f64>, num: Result<f64, ParseFloatError>) {
    match num {
        Ok(num) => stack.push(num),
        Err(e) => println!("Cannot push:  {e}."),
    }
}

fn small_stack_err(count_expected: usize, func: &str) {
    println!("Stack must contain at least {count_expected} numbers to {func}!");
}

fn operands_needed(func: &Function) -> usize {
    match func {
        Function::Add => 2,
        Function::Subtract => 2,
        Function::Multiply => 2,
        Function::Divide => 2,
        Function::Modulo => 2,
        Function::Floor => 2,
        Function::Ceil => 2,
        Function::Round => 1,
        Function::Abs => 1,
        Function::Pow => 2,
        Function::Sqrt => 1,
        Function::Cbrt => 1,
        Function::LogN => 1,
        Function::Log => 1,
        Function::Sin => 1,
        Function::Asin => 1,
        Function::Sinh => 1,
        Function::Acos => 1,
        Function::Cos => 1,
        Function::Cosh => 1,
        Function::Tan => 1,
        Function::Atan => 1,
        Function::Tanh => 1,
        Function::Epsilon => 0,
        Function::Euler => 0,
        Function::Pi => 0,
        Function::Tau => 0,
        Function::Push(_) => 0,
        Function::ClearStack => 0,
        Function::DeleteLast => 1,
        Function::Help => 0,
        Function::Quit => 0,
    }
}

fn operate(stack: &mut Vec<f64>, func: Function) {
    if stack.len() < operands_needed(&func) {
        small_stack_err(operands_needed(&func), format!("{func:?}").as_str());
    }
    let x;
    let y;
    //let z;

    match operands_needed(&func) {
        0 => {
            x = 0f64;
            y = 0f64;
            //z = 0f64;
        }
        1 => {
            x = stack.pop().unwrap_or(0f64);
            y = 0f64;
            //z = 0f64;
        }
        2 => {
            x = stack.pop().unwrap_or(0f64);
            y = stack.pop().unwrap_or(0f64);
            //z = 0f64;
        }
        3 => {
            x = stack.pop().unwrap_or(0f64);
            y = stack.pop().unwrap_or(0f64);
            //z = stack.pop().unwrap_or(0f64);
        }
        _ => {
            x = 0f64;
            y = 0f64;
            //z = 0f64;
            println!("Function requires more operands than I know how to pull off of the stack :(");
        }
    }

    match func {
        Function::Add => stack.push(x + y),
        Function::Subtract => stack.push(x - y),
        Function::Multiply => stack.push(x * y),
        Function::Divide => stack.push(y / x),
        //x is at top of stack, but divide is order
        //dependent. We're dividing the number input prior
        //to x (y), by x. i.e. x: 3, y: 6, push = 0.5
        Function::Modulo => stack.push(y % x),
        //same reason as division above
        Function::Floor => stack.push(floor(x, y)),
        Function::Ceil => stack.push(ceil(x, y)),
        Function::Round => stack.push(x.round()),
        Function::Abs => stack.push(x.abs()),
        Function::Pow => stack.push(y.powf(x)),
        Function::Sqrt => stack.push(x.sqrt()),
        Function::Cbrt => stack.push(x.cbrt()),
        Function::LogN => stack.push(x.ln()),
        Function::Log => stack.push(x.log10()),
        Function::Sin => stack.push(x.sin()),
        Function::Asin => stack.push(x.asin()),
        Function::Sinh => stack.push(x.sinh()),
        Function::Acos => stack.push(x.acos()),
        Function::Cos => stack.push(x.cos()),
        Function::Cosh => stack.push(x.cosh()),
        Function::Tan => stack.push(x.tan()),
        Function::Atan => stack.push(x.atan()),
        Function::Tanh => stack.push(x.tanh()),
        Function::Epsilon => stack.push(f64::EPSILON),
        Function::Euler => stack.push(std::f64::consts::E),
        Function::Pi => stack.push(std::f64::consts::PI),
        Function::Tau => stack.push(std::f64::consts::TAU),
        Function::Push(_) => println!("Push failed"),
        //this shouldn't happen, as this should be pushed from main()
        Function::Quit => println!("Quit failed"),
        //this shouldn't happen, as this should be breaking in main()
        Function::DeleteLast => {}
        //the last value was popped, but we aren't doing anything with it
        Function::ClearStack => stack.clear(),
        Function::Help => print_help(),
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

fn print_help() {
    println!("RPN Calculator Help");
    println!();
    println!("This is an RPN (Reverse Polish Notation) calculator program. This works by pushing numbers to a stack, and then completing operations on the pushed numbers in the order that they were added to the stack.");
    println!();
    println!("Example: 4 [enter] 3 [enter] * [enter]");
    println!("Will display 12 on the Current Stack (i.e. 4 * 3 = 12)");
    println!();
    println!("Many more math functions are available:");
    println!();
    println!("[Command]            [Function]");
    println!("q                    quit calculator (ctrl-c also works)");
    println!("+                    add the last two numbers on the stack");
    println!("-                    subtract the last two numbers on the stack");
    println!("* or x               multiply the last two numbers on the stack");
    println!("/                    divide the last two numbers on the stack");
    println!("%                    output the remainder of the division of the last two numbers on the stack (modulus)");
    println!("clear or c           clear the stack of all values");
    println!("del or d             remove the last number pushed to the stack (this can be repeated multiple times)");
    println!("floor                take the last two numbers from the stack, and push the lower number back onto the stack");
    println!("ceil                 take the last two numbers from the stack, and push the higher number back onto the stack");
    println!("round                round the last number on the stack (follows conventional rules, 0.5 rounds up)");
    println!("abs                  change the last number on the stack to it's absolute value");
    println!("pow                  raise the number on the stack prior to the last input to the power of the last number input");
    println!("                         Example: 3 [enter] 2 [enter] pow [enter] Output: 9");
    println!("sqrt or v            change the last number on the stack to it's square root");
    println!("cbrt or v3           change the last number on the stack to it's cube root");
    println!("ln                   change the last number on the stack to it's natural log");
    println!("log                  change the last number on the stack to it's base10 log");
    println!(
        "sin                  change the last number on the stack to it's sine value (radians)"
    );
    println!("asin, sinh               the same thing, but arc and hyperbolic instead");
    println!(
        "cos                  change the last number on the stack to it's cosine value (radians)"
    );
    println!("acos, cosh               the same thing, but arc and hyperbolic instead");
    println!(
        "tan                  change the last number on the stack to it's tangent value (radians)"
    );
    println!("atan, tanh               the same thing, but arc and hyperbolic instead");
    println!("me                   pushes the machine epsilon value onto the stack");
    println!("pi                   pushes pi onto the stack");
    println!("tau                  pushes tau on to the stack");
    println!("help or ?            print this help text");
    println!();
    println!();
}
