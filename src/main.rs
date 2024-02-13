#![allow(unused)]

use serde::Deserialize;
use std::io::{stdin, Read};

#[derive(Debug, Deserialize)]
pub struct File {
    name: String,
    expression: Term,
}

#[derive(Debug, Deserialize)]
pub struct Int {
    value: i32,
}

#[derive(Debug, Deserialize)]
pub struct Str {
    value: String,
}

#[derive(Debug, Deserialize)]
pub struct Print {
    value: Box<Term>,
}

#[derive(Debug, Deserialize)]
pub struct Binary {
    rhs: Box<Term>,
    op: BinaryOp,
    lhs: Box<Term>,
}

#[derive(Debug, Deserialize)]
pub enum BinaryOp {
    Add,
    Sub,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
pub enum Term {
    Int(Int),
    Str(Str),
    Print(Print),
    Binary(Binary),
}

#[derive(Debug)]
pub enum Val {
    Void,
    Int(i32),
    Bool(bool),
    Str(String),
}

fn eval(term: Term) -> Val {
    match term {
        Term::Int(number) => Val::Int(number.value),
        Term::Str(str) => Val::Str(str.value),
        Term::Print(print) => {
            let val = eval(*print.value);
            match val {
                Val::Int(number) => print!("{}", number),
                Val::Str(str) => print!("{}", str),
                Val::Bool(bool) => print!("{}", bool),
                _ => panic!("Valor não suportado"),
            }
            Val::Void
        }
        Term::Binary(binary) => {
            let lhs = eval(*binary.lhs);
            let rhs = eval(*binary.rhs);
            match binary.op {
                BinaryOp::Add => match (lhs, rhs) {
                    (Val::Int(lhs), Val::Int(rhs)) => Val::Int(lhs + rhs),
                    (Val::Str(lhs), Val::Int(rhs)) => Val::Str(format!("{lhs}{rhs}")),
                    (Val::Int(lhs), Val::Str(rhs)) => Val::Str(format!("{lhs}{rhs}")),
                    (Val::Str(lhs), Val::Str(rhs)) => Val::Str(format!("{lhs}{rhs}")),
                    _ => panic!("Operação inválida"),
                },
                BinaryOp::Sub => match (lhs, rhs) {
                    (Val::Int(lhs), Val::Int(rhs)) => Val::Int(lhs - rhs),
                    _ => panic!("Operação inválida"),
                },
            }
        }
    }
}

fn main() {
    let mut program = String::new();
    stdin().lock().read_to_string(&mut program).unwrap();
    let program = serde_json::from_str::<File>(&program).unwrap();

    let term = program.expression;
    eval(term);
}
