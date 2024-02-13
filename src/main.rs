#![allow(unused)]

use serde::Deserialize;
use std::{
    collections::HashMap,
    io::{stdin, Read},
};

#[derive(Debug, Deserialize)]
pub struct File {
    name: String,
    expression: Term,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Int {
    value: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Str {
    value: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Bool {
    value: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Print {
    value: Box<Term>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Binary {
    rhs: Box<Term>,
    op: BinaryOp,
    lhs: Box<Term>,
}

#[derive(Debug, Deserialize, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Lt,
}

#[derive(Debug, Deserialize, Clone)]
pub struct If {
    condition: Box<Term>,
    then: Box<Term>,
    otherwise: Box<Term>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Parameter {
    text: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Let {
    name: Parameter,
    value: Box<Term>,
    next: Box<Term>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Var {
    text: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Function {
    parameters: Vec<Parameter>,
    value: Box<Term>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Call {
    callee: Box<Term>,
    arguments: Vec<Term>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum Term {
    Int(Int),
    Str(Str),
    Bool(Bool),
    Print(Print),
    Binary(Binary),
    If(If),
    Let(Let),
    Var(Var),
    Function(Function),
    Call(Call),
}

#[derive(Debug, Clone)]
pub enum Val {
    Void,
    Int(i32),
    Bool(bool),
    Str(String),
    Closure {
        body: Term,
        params: Vec<Parameter>,
        env: Scope,
    },
}

pub type Scope = HashMap<String, Val>;

fn eval(term: Term, scope: &mut Scope) -> Val {
    match term {
        Term::Int(number) => Val::Int(number.value),
        Term::Str(str) => Val::Str(str.value),
        Term::Bool(bool) => Val::Bool(bool.value),
        Term::Print(print) => {
            let val = eval(*print.value, scope);
            match val {
                Val::Int(number) => print!("{}", number),
                Val::Str(str) => print!("{}", str),
                Val::Bool(bool) => print!("{}", bool),
                _ => panic!("Valor não suportado"),
            }
            Val::Void
        }
        Term::Binary(binary) => {
            let lhs = eval(*binary.lhs, scope);
            let rhs = eval(*binary.rhs, scope);
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
                BinaryOp::Lt => match (lhs, rhs) {
                    (Val::Int(lhs), Val::Int(rhs)) => Val::Bool(lhs < rhs),
                    _ => panic!("Operação inválida"),
                },
            }
        }
        Term::If(i) => match eval(*i.condition, scope) {
            Val::Bool(true) => eval(*i.then, scope),
            Val::Bool(false) => eval(*i.otherwise, scope),
            _ => panic!("Condição inválida"),
        },
        Term::Let(l) => {
            let name = l.name.text;
            let value = eval(*l.value, scope);
            scope.insert(name, value);
            eval(*l.next, scope)
        }
        Term::Var(v) => match scope.get(&v.text) {
            Some(val) => val.clone(),
            None => panic!("Variável não encontrada"),
        },
        Term::Function(f) => Val::Closure {
            body: *f.value,
            params: f.parameters,
            env: scope.clone(),
        },
        Term::Call(c) => {
            let callee = eval(*c.callee, scope);
            let mut args = Vec::new();
            for arg in c.arguments {
                args.push(eval(arg, scope));
            }
            match callee {
                Val::Closure { body, params, env } => {
                    if params.len() != args.len() {
                        panic!("Número de argumentos inválido");
                    }
                    let mut new_scope = scope.clone();
                    for (param, arg) in params.into_iter().zip(args) {
                        new_scope.insert(param.text, arg);
                    }
                    eval(body, &mut new_scope)
                }
                _ => panic!("Chamada inválida"),
            }
        }
    }
}

fn main() {
    let mut program = String::new();
    stdin().lock().read_to_string(&mut program).unwrap();
    let program = serde_json::from_str::<File>(&program).unwrap();

    let term = program.expression;
    let mut scope = HashMap::new();
    eval(term, &mut scope);
}
