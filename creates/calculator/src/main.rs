use std::env::{self, Args};
use std::process;
use std::cmp::Ordering;


fn main() {
    
}

enum Element {
    Number(f64),
    Operator(Operator),
}

enum Operator {
    LeftBracket,
    RightBracket,
    Plus,
    Minus,
    Multiply,
    Divide,
}
impl Operator {
    fn get_priority(&self) -> u16{
        match self{
            Operator::Plus | Operator::Minus => 1,
            Operator::Multiply | Operator::Divide => 2,
            _ => 0,
        }
    }
}
    
fn parse(input: &str) -> Result<Element,&'static str>{
    match input{
        "(" => return Ok(Element::Operator(Operator::LeftBracket)),
        ")" => return Ok(Element::Operator(Operator::RightBracket)),
        "+" => return Ok(Element::Operator(Operator::Plus)),
        "-" => return Ok(Element::Operator(Operator::Minus)),
        "*" => return Ok(Element::Operator(Operator::Multiply)),
        "/" => return Ok(Element::Operator(Operator::Divide)),
        _ => (),
    };
    match input.parse() {
        Ok(num) => return Ok(Element::Number(num)),
        Err(_) => return Err("Error"),
    };
}
fn convert_to_vec(args:Args) -> Vec<Element> {
    let mut elements:Vec<Element> = Vec::new();
    for i in args{
        elements.push(parse(&i).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1)}));
    }
    elements
}
fn run(elements:Vec<Element>) -> Vec<Element> {
    let mut reverse_polish_notation:Vec<Element> = Vec::new();
    let mut operators:Vec<Operator> = Vec::new();
    for element in elements{
        match element {
            Element::Number(_) => reverse_polish_notation.push(i),
            Element::Operator(operator) =>
                match operator {
                    Operator::LeftBracket => operators.push(operator),
                    Operator::RightBracket => loop {
                        match operators.pop() {
                            Some(operator) => reverse_polish_notation.push(Element::Operator(operator)),
                            None | Some(Operator::LeftBracket) => break,
                        }
                    },
                    other_operator => match operators.last().unwrap().get_priority().cmp(&other_operator.get_priority()) { 
                        Ordering::Greater => operators.push(other_operator),
                        _ => ,
                    },
                    }
            }
        }
    reverse_polish_notation
}

trait Stack<T> {
    fn top(&self) -> &T;
    fn push(t:T) -> ();

}
impl<T> Stack<T> for Vec<T>{
    fn top(&self) -> &T {
        return self.last().unwrap();
    }
    fn push(t:T) -> () {
        
    }
}