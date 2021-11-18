mod passgen;
use argh::FromArgs;

#[derive(FromArgs)]
/// A password generator
struct Cli {
    #[argh(option, description = "password length", short = 'l', default = "16")]
    length: i32,

    #[argh(switch, description = "switch for alphabets", short = 'a')]
    alphabets: bool,

    #[argh(switch, description = "switch for caps", short = 'c')]
    caps: bool,

    #[argh(switch, description = "switch for numbers", short = 'n')]
    numbers: bool,

    #[argh(switch, description = "switch for symbols", short = 's')]
    symbols: bool,
}

fn main() {
    let cli: Cli = argh::from_env();
    
    let length      = cli.length.clone();
    let alphabets   = cli.alphabets.clone();
    let numbers     = cli.numbers.clone();
    let symbols     = cli.symbols.clone();
    let caps        = cli.caps.clone();
    
    if alphabets == false && numbers == false && symbols == false  {
        println!("Please select atleast one option. do \"--help\" for more info");
        return;
    }
    let passgen = passgen::PasswordGenerator::new(length, alphabets, numbers, symbols, caps);

    println!("{}", passgen.generate());
}