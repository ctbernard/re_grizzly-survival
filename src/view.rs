use std::{env, io};
use std::env::args;
use std::io::Read;

pub fn parseArguments() -> Vec<String>{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let split= input.split_whitespace();
    let split_input: Vec<String> = split.map(|x| x.to_string()).collect();
    split_input
}
