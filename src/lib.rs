use indoc::printdoc;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
pub type Value = f64;
pub type Result = std::result::Result<(), Error>;

pub struct RPN {
    custom: HashMap<String, Definition>,
    stack: Vec<Value>,
}

#[derive(Debug)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
    StackPushFailed,
}

#[derive(Clone, Debug)]
struct Definition {
    words: Arc<Vec<Words>>,
}

// https://www.forth.com/starting-forth/2-stack-manipulation-operators-arithmetic/
// for definition of swap, dup, over, drop operators
#[derive(Debug)]
enum Words {
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
    Push(Value),
    Drop,
    Duplicate,
    Swap,
    Over,
    ClearStack,
    Emit,
    Sum,
    DefineStart,
    DefineEnd,
    Definition(Definition),
    EndOfLine,
    Help,
    Quit,
}

impl Default for RPN {
    fn default() -> Self {
        RPN::new()
    }
}

impl RPN {
    pub fn new() -> RPN {
        RPN {
            custom: HashMap::new(),
            stack: Vec::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    fn add(&mut self) -> Result {
        let x = self.pop()?;
        let y = self.pop()?;

        self.push(y + x)
    }

    fn subtract(&mut self) -> Result {
        let x = self.pop()?;
        let y = self.pop()?;

        self.push(y - x)
    }

    fn multiply(&mut self) -> Result {
        let x = self.pop()?;
        let y = self.pop()?;

        self.push(y * x)
    }

    fn divide(&mut self) -> Result {
        let x = self.pop()?;
        let y = self.pop()?;

        if x == 0f64 {
            Err(Error::DivisionByZero)
        } else {
            self.push(y / x)
        }
    }

    fn push(&mut self, num: Value) -> Result {
        self.stack.push(num);
        //push can fail, but I have no idea how to actually catch that.
        //This should only happen on an out of memory error anyway
        Ok(())
    }

    fn pop(&mut self) -> std::result::Result<Value, Error> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }

    fn duplicate(&mut self) -> Result {
        let x = self.pop()?;

        self.push(x)?;
        self.push(x)
    }

    fn drop(&mut self) -> Result {
        let _ = self.pop()?;

        Ok(())
    }

    fn swap(&mut self) -> Result {
        let x = self.pop()?;
        let y = self.pop()?;

        self.push(x)?;
        self.push(y)
    }

    fn over(&mut self) -> Result {
        let x = self.pop()?;
        let y = self.pop()?;

        self.push(y)?;
        self.push(x)?;
        self.push(y)
    }

    fn modulo(&mut self) -> Result {
        let x = self.pop()?;
        let y = self.pop()?;

        self.push(y % x)
    }

    fn abs(&mut self) -> Result {
        let x = self.pop()?;

        self.push(x.abs())
    }

    fn floor(&mut self) -> Result {
        let x = self.pop()?;
        let y = self.pop()?;

        self.push(RPN::floor_cmp(x, y))
    }

    fn floor_cmp(a: Value, b: Value) -> Value {
        if a > b {
            a
        } else {
            b
        }
    }

    fn ceil(&mut self) -> Result {
        let x = self.pop()?;
        let y = self.pop()?;

        self.push(RPN::ceil_cmp(x, y))
    }

    fn ceil_cmp(a: Value, b: Value) -> Value {
        if a < b {
            a
        } else {
            b
        }
    }
    fn round(&mut self) -> Result {
        let x = self.pop()?;

        self.push(x.round())
    }
    fn pow(&mut self) -> Result {
        let x = self.pop()?;
        let y = self.pop()?;

        self.push(y.powf(x))
    }

    fn sqrt(&mut self) -> Result {
        let x = self.pop()?;

        self.push(x.sqrt())
    }

    fn cbrt(&mut self) -> Result {
        let x = self.pop()?;

        self.push(x.cbrt())
    }

    fn log_natural(&mut self) -> Result {
        let x = self.pop()?;

        self.push(x.ln())
    }

    fn log_base10(&mut self) -> Result {
        let x = self.pop()?;

        self.push(x.log10())
    }

    fn sin(&mut self) -> Result {
        let x = self.pop()?;

        self.push(x.sin())
    }

    fn asin(&mut self) -> Result {
        let x = self.pop()?;

        self.push(x.asin())
    }

    fn sinh(&mut self) -> Result {
        let x = self.pop()?;

        self.push(x.sinh())
    }

    fn cos(&mut self) -> Result {
        let x = self.pop()?;

        self.push(x.cos())
    }

    fn acos(&mut self) -> Result {
        let x = self.pop()?;

        self.push(x.acos())
    }

    fn cosh(&mut self) -> Result {
        let x = self.pop()?;

        self.push(x.cosh())
    }

    fn tan(&mut self) -> Result {
        let x = self.pop()?;

        self.push(x.tan())
    }

    fn atan(&mut self) -> Result {
        let x = self.pop()?;

        self.push(x.atan())
    }

    fn tanh(&mut self) -> Result {
        let x = self.pop()?;

        self.push(x.tanh())
    }

    fn emit(&mut self) -> Result {
        let x = self.pop()?;

        println!("{x}");

        Ok(())
    }

    fn sum(&mut self) -> Result {
        let mut accumulator = self.pop()?;

        while let Ok(x) = self.pop() {
            accumulator += x;
        }

        self.push(accumulator)
    }

    fn define(&mut self, name: &str, definition: Definition) {
        let name = name.to_lowercase();
        self.custom.insert(name, definition);
    }

    fn do_definition(&mut self, definition: &Definition) -> Result {
        let words_iter = Arc::clone(&definition.words);
        for w in &*words_iter {
            match self.do_word(w) {
                Err(e) => return Err(e),
                Ok(_) => continue,
            }
        }
        Ok(())
    }

    fn do_word(&mut self, word: &Words) -> Result {
        match word {
            Words::Add => self.add(),
            Words::Subtract => self.subtract(),
            Words::Multiply => self.multiply(),
            Words::Divide => self.divide(),
            Words::Duplicate => self.duplicate(),
            Words::Drop => self.drop(),
            Words::Swap => self.swap(),
            Words::Over => self.over(),
            Words::Push(x) => self.push(*x),
            Words::Modulo => self.modulo(),
            Words::Abs => self.abs(),
            Words::Floor => self.floor(),
            Words::Ceil => self.ceil(),
            Words::Round => self.round(),
            Words::Pow => self.pow(),
            Words::Sqrt => self.sqrt(),
            Words::Cbrt => self.cbrt(),
            Words::LogN => self.log_natural(),
            Words::Log => self.log_base10(),
            Words::Sin => self.sin(),
            Words::Sinh => self.sinh(),
            Words::Asin => self.asin(),
            Words::Cos => self.cos(),
            Words::Cosh => self.cosh(),
            Words::Acos => self.acos(),
            Words::Tan => self.tan(),
            Words::Atan => self.atan(),
            Words::Tanh => self.tanh(),
            Words::Emit => self.emit(),
            Words::Sum => self.sum(),
            Words::Epsilon => self.push(f64::EPSILON),
            Words::Euler => self.push(std::f64::consts::E),
            Words::Pi => self.push(std::f64::consts::PI),
            Words::Tau => self.push(std::f64::consts::TAU),
            Words::ClearStack => {
                self.stack.clear();
                Ok(())
            }
            Words::Definition(x) => self.do_definition(x),
            Words::Help => {
                RPN::print_help();
                Ok(())
            }
            Words::Quit => std::process::exit(0),
            //definitions and line endings are special, and must be handled by eval()
            Words::DefineStart => Err(Error::InvalidWord),
            Words::DefineEnd => Err(Error::InvalidWord),
            Words::EndOfLine => Err(Error::InvalidWord),
        }
    }

    fn parse(&self, cmd: &str) -> std::result::Result<Words, Error> {
        let q = cmd.to_ascii_lowercase();
        let def = self.custom.get(&q);
        if let Some(x) = def {
            return Ok(Words::Definition(x.clone()));
        }
        match q.as_str() {
            "+" => Ok(Words::Add),
            "-" => Ok(Words::Subtract),
            "*" => Ok(Words::Multiply),
            "/" => Ok(Words::Divide),
            "dup" => Ok(Words::Duplicate),
            "drop" => Ok(Words::Drop),
            "swap" => Ok(Words::Swap),
            "over" => Ok(Words::Over),
            ":" => Ok(Words::DefineStart),
            ";" => Ok(Words::DefineEnd),
            "%" => Ok(Words::Modulo),
            "abs" | "||" => Ok(Words::Abs),
            "floor" | "fl" => Ok(Words::Floor),
            "ceil" | "cl" => Ok(Words::Ceil),
            "round" | "r" => Ok(Words::Round),
            "pow" | "^" => Ok(Words::Pow),
            "sqrt" | "v" => Ok(Words::Sqrt),
            "cbrt" | "v3" => Ok(Words::Cbrt),
            "ln" => Ok(Words::LogN),
            "log" => Ok(Words::Log),
            "sin" => Ok(Words::Sin),
            "asin" => Ok(Words::Asin),
            "sinh" => Ok(Words::Sinh),
            "cos" => Ok(Words::Cos),
            "acos" => Ok(Words::Acos),
            "cosh" => Ok(Words::Cosh),
            "tan" => Ok(Words::Tan),
            "atan" => Ok(Words::Atan),
            "tanh" => Ok(Words::Tanh),
            "me" => Ok(Words::Epsilon),
            "euler" | "e" => Ok(Words::Euler),
            "pi" => Ok(Words::Pi),
            "tau" => Ok(Words::Tau),
            "help" | "?" => Ok(Words::Help),
            "emit" | "." => Ok(Words::Emit),
            "sum" => Ok(Words::Sum),
            "clear" | "c" => Ok(Words::ClearStack),
            "quit" | "q" => Ok(Words::Quit),
            unk => {
                //determine if input is a number, if it is, push it to stack
                //otherwise, return UnknownWord error
                let num = Value::from_str(unk);

                if let Ok(x) = num {
                    return Ok(Words::Push(x));
                }
                Err(Error::UnknownWord)
            }
        }
    }

    fn get_next(&self, iter: &mut core::str::Split<'_, char>) -> std::result::Result<Words, Error> {
        let next = iter.next();
        match next {
            None => Ok(Words::EndOfLine),
            Some(cmd) => self.parse(cmd),
        }
    }

    pub fn do_define(&mut self, iter: &mut core::str::Split<'_, char>) -> Result {
        let name: &str = match iter.next() {
            None => return Err(Error::InvalidWord),
            Some(x) => x,
        };

        //numbers can not be names of definitions
        //if the parse succeeds (indicating that the input name is a number), error out
        if Value::from_str(name).is_ok() {
            return Err(Error::InvalidWord);
        }

        let mut def_working: Vec<Words> = Vec::new();
        loop {
            match self.get_next(iter) {
                Err(e) => return Err(e),
                Ok(w) => match w {
                    Words::DefineStart => {
                        //can not begin a new definition before this definition has completed
                        return Err(Error::InvalidWord);
                    }
                    Words::DefineEnd => {
                        let def: Definition = Definition {
                            words: Arc::new(def_working),
                        };
                        self.define(name, def);
                        return Ok(());
                    }
                    Words::EndOfLine => {
                        //line can not end before definition is completed
                        return Err(Error::InvalidWord);
                    }
                    word => {
                        def_working.push(word);
                    }
                },
            }
        }
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut input_vec = input.trim_end().split(' ');

        loop {
            let cur = self.get_next(&mut input_vec);

            match cur {
                Ok(Words::EndOfLine) => break Ok(()),
                Ok(Words::DefineStart) => self.do_define(&mut input_vec)?,
                Ok(word) => self.do_word(&word)?,
                Err(e) => break Err(e),
            }
        }
    }

    fn print_help() {
        printdoc! {"
        RPN Calculator Help

        This is an RPN (Reverse Polish Notation) calculator program. This works by pushing numbers to a stack, and then completing operations on the pushed numbers in the order that they were added to the stack.

        Inputs can be chained together by inputting commands seperated by a space.

        Example: 4 [enter] 3 [enter] * [enter]
        Will display 12 on the Current Stack (i.e. 4 * 3 = 12)

        Example: 4 [space] 3 [space] * [enter]
        Will also display 12 on the Current Stack

        Many more math functions are available:

        [Command]            [Function]
        quit or q            quit calculator (ctrl-c also works)
        +                    add the last two numbers on the stack
        -                    subtract the last two numbers on the stack
        * or x               multiply the last two numbers on the stack
        /                    divide the last two numbers on the stack
        %                    output the remainder of the division of the last two numbers on the stack (modulus)
        clear or c           clear the stack of all values
        drop or d            remove the last number pushed to the stack (this can be repeated multiple times)
        floor                take the last two numbers from the stack, and push the lower number back onto the stack
        ceil                 take the last two numbers from the stack, and push the higher number back onto the stack
        round                round the last number on the stack (follows conventional rules, 0.5 rounds up)
        abs                  change the last number on the stack to it's absolute value
        pow                  raise the number on the stack prior to the last input to the power of the last number input
                                 Example: 3 [enter] 2 [enter] pow [enter] Output: 9
        sqrt or v            change the last number on the stack to it's square root
        cbrt or v3           change the last number on the stack to it's cube root
        ln                   change the last number on the stack to it's natural log
        log                  change the last number on the stack to it's base10 log
        sin                  change the last number on the stack to it's sine value (radians)
        asin, sinh               the same thing, but arc and hyperbolic instead
        cos                  change the last number on the stack to it's cosine value (radians)
        acos, cosh               the same thing, but arc and hyperbolic instead
        tan                  change the last number on the stack to it's tangent value (radians)
        atan, tanh               the same thing, but arc and hyperbolic instead
        me                   pushes the machine epsilon value onto the stack
        pi                   pushes pi onto the stack
        tau                  pushes tau on to the stack
        help or ?            print this help text

        emit or .            print the last number pushed to the stack, removing it from the stack

        sum                  sum the entire stack, pushing the result back onto the stack


            Additionally, this implements a subset of FORTH for a basic level of programability.

            The ':' character is used to begin a definition, and a definition is ended with ';'

            Example: : name 1 2 3 + + ;

            This creates a definition of 'name' that can then be called after it's definition.

            The definition pushes 1, 2, 3, onto the stack, and then adds 3 + 2, and then 5 + 1.

            Definitions can be used inside other definitions:

            Example: : name2 name name name ;

            Creates a definition 'name2' that executes the 'name' definition 3 times.



        "}
    }
}
