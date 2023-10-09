use rpn::RPN;

use simple_repl::*;

fn main() {
    println!("RPN Calculator (with FORTH)");

    let mut rpn = RPN::new();
    //we do not want to exit if rpn.eval returns an error, we just want to print it for the user
    //no need to use anything other than () for passthrough and error types
    let mut eval = |input: &str|-> Result<EvalResult<()>, ()> {
        match rpn.eval(input) {
            Err(e) => println!("Failed: {e:?}"),
            Ok(_) => {
                println!("Current Stack: ");
                for n in rpn.stack() {
                    println!("{n}");
                }
            }
        }
        //no need to have simple-repl do anything but loop forever with this application
        Ok(EvalResult::Continue)
    };

    //simple-repl handles the loop for us
    let _ = repl(&mut eval);
}


