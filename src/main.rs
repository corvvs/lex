use std::env;
use std::io::{ self };

use ft_lex::structures::Yo;
mod input_parser;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut yo = Yo::default();

    if args.len() > 1 {
        yo.config.input_path = Some(args[1].clone());
    }

    // しくじったら勝手にエラーが返る
    input_parser::parse_input(&mut yo)?;

    println!("yo = {:#?}", yo);

    Ok(())
}
