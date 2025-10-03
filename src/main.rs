use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::io::Result;

mod ast;

lalrpop_mod!(sysy);

fn main() -> Result<()> {
    let mut args = args();
    args.next(); // skip executable name
    let input = args.next().unwrap();
    let output = args.next().unwrap();

    let input = read_to_string(input)?;

    let ast = sysy::CompUnitParser::new().parse(&input).unwrap();

    // std::fs::write(output, ast.as_bytes())?;
    std::fs::write(output, format!("{:#?}", ast).as_bytes())?;
    Ok(())
}
