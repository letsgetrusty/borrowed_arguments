trait Animal {
    fn speak(&self);
}

#[derive(Debug)]
struct Cat {
    name: String
}

impl Animal for Cat {
    fn speak(&self) {
        println!("meow!");
    }
}

#[derive(Debug)]
struct Dog {
    name: String
}

impl Animal for Dog {
    fn speak(&self) {
        println!("woof!");
    }
}

fn main() {
    let peanut = "peanut".to_owned();
    let oreo = "oreo".to_owned();
    let jax = "jax";

    print_animal_name(&oreo);
    print_animal_name(jax);

    let cat = Box::new(Cat { name: peanut });
    let dog = Box::new(Dog { name: oreo });
    let dog2 = Dog { name: jax.to_owned() };

    print_dog(&dog);
    print_dog(&dog2);

    let animals: Vec<Box<dyn Animal>> = vec![cat, dog];
    // let animals: [Box<dyn Animal>; 2] = [cat, dog];

    animal_sounds(&animals);
}

// fn print_animal_name(name: &String) {
//     println!("{name}");
// }

// fn print_dog(dog: &Box<Dog>) {
//     println!("{:?}", dog);
// }

// fn animal_sounds(animals: &Vec<Box<dyn Animal>>) {
//     for a in animals {
//         a.speak();
//     }
// }

// Prefer using targets of deref coercion to: 
// 1. Increase flexibility of code (allow more types as arguments)
// 2. Avoid extra layers of indirection

fn print_animal_name(name: &str) {
    println!("{name}");
}

fn print_dog(dog: &Dog) {
    println!("{:?}", dog);
}

fn animal_sounds(animals: &[Box<dyn Animal>]) {
    for a in animals {
        a.speak();
    }
}