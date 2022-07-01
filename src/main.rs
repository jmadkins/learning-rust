use my_name::hello;
use person::*;
mod person;

fn main() {
    println!("Hello, world!");
    let foo = "pop";
    println!("{arg1} {}", my_new_name(), arg1 = foo);
    let mut my_person = Person::new("Justin", 27);
    println!("{}", my_person);
    my_person.announce();
    my_person.print_status();
    my_person.set_name("Jew");
    let dave = String::from("Dave");
    my_person.set_name(&dave);

    println!("{}", my_person);

    let my_data = vec![1, 4, 9];
    let other_data = my_data.iter().map(|j| j + 5);
    for data in other_data {
        println!("{}", data);
    }

    my_name::hello();
    hello();

    something(my_person.clone());
    println!("{}", my_person);
}

fn my_new_name() -> String {
    String::from("My value")
}

mod my_name {
    pub fn hello() {
        println!("Hello!")
    }
}

fn something_cool(dave: &mut Person) {
    println!("{}", dave);
}

fn something(mut dave: Person) -> Person {
    dave.set_name("dave");
    dave
}
