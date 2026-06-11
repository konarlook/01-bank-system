use parslib::error::ConvertError;
use parslib::format::Format;
use parslib::{compare, read_transactions};
use std::error::Error;
use std::path::Path;
use std::{env, fs};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Использование");
        eprintln!("     comparer first.csv second.txt");
        eprintln!("     comparer first.txt second.bin");
        eprintln!("     comparer first.bin seconds.csv");
        return Ok(());
    }

    let inp_1: Format = Path::new(&args[1])
        .extension()
        .and_then(|e| e.to_str())
        .ok_or(ConvertError::UnknownFormat)?
        .parse()?;

    let inp_2: Format = Path::new(&args[2])
        .extension()
        .and_then(|e| e.to_str())
        .ok_or(ConvertError::UnknownFormat)?
        .parse()?;

    let mut first_reader = fs::File::open(&args[1].clone())?;
    let mut second_reader = fs::File::open(&args[2].clone())?;

    let f = read_transactions(inp_1, &mut first_reader)?;
    let s = read_transactions(inp_2, &mut second_reader)?;

    match compare(&f, &s) {
        Some(diff) => eprintln!("{}", diff),
        None => println!("Транзакции полностью совпадают"),
    }
    Ok(())
}
