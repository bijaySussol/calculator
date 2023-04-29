mod calculator;
use std::io;

use calculator::Error;
fn main() -> Result<(), Error> {
    // println!("Hello, world!");
    // let tokens = calculator::Calculator::parse("2 * 2 + 48 /4");
    // println!("{:?}",tokens);
    // let expr = calculator::Calculator::expression(tokens.unwrap());
    // println!("{:?}", expr);
    // let value = calculator::Calculator::evaluate(expr);
    // println!("{:?}", value);
    loop {
        let mut input = String::new();
        println!("Enter number");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let token = calculator::Calculator::parse(input);
                if token.is_err() {
                    println!("{:?}",token.err().unwrap());
                    continue;
                }
                let expr = calculator::Calculator::expression(token?);
                if let Some(value) = calculator::Calculator::evaluate(expr) {
                    println!("{}", value);
                }
            }
            Err(error) => println!("Error on reading calculator: {}",error),
        }
    }
}
