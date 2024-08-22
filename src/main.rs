// #![allow(warnings)]

mod tests;

mod interpreter;
mod scanner;
mod parser;
mod token;
mod errors;
mod grammar;
mod reader;

fn welcome() {
    let mut message: String = String::new();
    message += "\nPropositional algebra evaluator\n\n";
    message += "Write a proposition and it will be evaluated.\n";
    message += "----------------------------\nSymbol list:\n";
    message += "and: &\nor: |\nnot: !\nif and only if: ~\nif, then: >\n----------------------------\n";
    message += "Any alphabetical letter will be interpreted as a simple proposition.\n";
    message += "You can group using parenthesis.\n";
    message += "Write your expression in the next line:\n";
    println!("{}", message);
}

fn main() {
    welcome();

    let proposition = reader::read_expression();
    if proposition.is_empty() {
        println!("Proposition is empty.");
        return
    }

    let (tokens, simples, err) = scanner::scan(&proposition);

    let res = parser::parse(tokens);

    println!();
    if err {
        println!("The proposition is not a well formed formula so is not possible to evaluate.");
        return
    } else if let Err(_) = res {
        println!("The proposition is not a well formed formula so is not possible to evaluate.");
        return
    } else {
        println!("Good! The proposition is a well formed formula.");
    }
    println!();

    let values = reader::read_simples(simples);
    println!();

    let eval = interpreter::interpret(values, res.unwrap());
    println!("Your proposition evaluates to: {}", eval);
}