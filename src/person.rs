#![allow(dead_code)]
use std::fmt;

/// Yo man, this is a person.
#[derive(Clone)]
pub struct Person {
    name: String,
    age: u8,
    status: Status,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "My name is {} and I'm {}", self.name, self.age)
    }
}

impl Person {
    pub fn new(name: &str, age: u8) -> Self {
        Self {
            name: String::from(name),
            age,
            status: Status::Closed {
                closer: String::from("My name"),
                id: 4,
            },
        }
    }

    pub fn announce(&self) {
        println!("I'm here!");
    }

    pub fn print_status(&self) {
        match &self.status {
            Status::Open(my_string) => println!("Open {}", my_string),
            Status::Closed { closer, id } => println!("Closed"),
        }
    }

    pub fn set_name(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }
}

#[derive(Clone)]
enum Status {
    Open(String),
    Closed { closer: String, id: u32 },
}

#[derive(Clone)]
enum Glass {
    Filled(String),
    Empty,
}

impl Status {
    fn close(&mut self) {
        match self {
            Status::Open(_) => {
                *self = Status::Closed {
                    closer: String::from("D"),
                    id: 1,
                }
            }
            _ => {}
        }
    }
}
