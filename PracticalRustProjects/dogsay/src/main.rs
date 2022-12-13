
/*
// 1st program
fn main() {
    let message = std::env::args().nth(1)
                    .expect("Missing input message. Usage: dogsay <message>");
    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("    /\\_/\\");
    println!("    ( o o )");
    println!("     ( = )");
}
*/

// 2nd program
extern crate structopt;
extern crate colored;

use structopt::StructOpt;
use colored::*;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Woof! woof!")]
    message: String,

    #[structopt(short = "d", long = "dead")]
    dead: bool,
}

fn main() {

    let options = Options::from_args();
    let message = options.message;

    let dog_eye = if options.dead {"x"} else {"o"};
    println!("{}", message.bright_yellow().underline());
    println!(" \\");
    println!("  \\");
    println!("    /\\___/\\");
    println!("    ( {eye} {eye} )", eye=dog_eye.red().bold());
    println!("     ( = )");
}