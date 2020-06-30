#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

trait Animal {
    // a function that accepts a string
    fn create(name: &'static str) -> Self;

    // an instance function name, which returns a string
    fn name(&self) -> &'static str;

    fn talk(&self) {
        // default behaviour of talk
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str
}

impl Animal for Human {
    fn create(name: &'static str) -> Human {
        Human { name: name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello!", self.name());
    }
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat { name: name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says meow!", self.name());
    }
}

pub fn traits() {
    /* let h1 = Human { name:"John" }; */
    /* let h1 = Human::create("John"); */
    let h1:Human = Animal::create("John");
    h1.talk();

    /* let c1 = Cat { name:"Bitch" }; */
    let c1 = Cat::create("Bitch");
    c1.talk();
}