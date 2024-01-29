use day3::builder_type::*;
use day3::trait_objects::*;
use day3::binary_tree::*;
use day3::health_statistics::*;

fn main() {

    println!("\n>>> builder_type <<<");
    let base64 = PackageBuilder::new("base64").version("0.13").build();
    println!("base64: {base64:?}");
    let log =
        PackageBuilder::new("log").version("0.4").language(Language::Rust).build();
    println!("log: {log:?}");
    let serde = PackageBuilder::new("serde")
        .authors(vec!["djmitche".into()])
        .version(String::from("4.0"))
        .dependency(base64.as_dependency())
        .dependency(log.as_dependency())
        .build();
    println!("serde: {serde:?}");


    println!("\n>>> trait_objects <<<");
    let pets: Vec<Box<dyn Pet>> = vec![
        Box::new(Dog {
            name: String::from("Azor"),
            age: 8,
        }),
        Box::new(Cat { lives: 9 }),
    ];

    for pet in pets {
        println!("pet.talk(): {}", pet.talk());
    }

    println!("{} {}", std::mem::size_of::<Dog>(), std::mem::size_of::<Cat>());
    println!("{} {}", std::mem::size_of::<&Dog>(), std::mem::size_of::<&Cat>());
    println!("{}", std::mem::size_of::<&dyn Pet>());
    println!("{}", std::mem::size_of::<Box<dyn Pet>>());


    println!("\n>>> binary_tree <<<");
    // Tests created in binary_tree.rs


    println!("\n>>> health_statistics <<<");
}
