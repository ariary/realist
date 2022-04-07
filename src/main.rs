use itertools::Itertools;
use itertools::free::join;
use std::fmt;
use clap::{Arg, Command};
use std::process;


pub struct Word {
    characters: Vec<u32>,
}

impl fmt::Display for Word {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&join(&self.characters[..], &""))?;
        Ok(())
    }
}


fn main() {
    let mut minlen: usize=0;
    // flag parsing
    let matches = Command::new("realist")
        .version("0.1.0")
        .author("ariary <nomail@nomail>")
        .about("Constraints-based wordlist generator")
        .arg(Arg::new("min-len")
                 .short('m')
                 .long("min-len")
                 .takes_value(true)
                 .help("minimum word len"))
        .get_matches();
    
    let minlength = matches.value_of("min-len");
    match minlength {
        None => minlen =4,  //default
        Some(s) => {
            match s.parse::<usize>() {
                Ok(n) => minlen = n,
                Err(_) =>{
                    eprintln!("--min-len ({}) is not a number", s);
                    process::exit(1);
                } 
            }
        }
    }
    let numbers: Vec<u32> = (0..10).collect();

    let my_scores = Word {
        characters: vec![12, 23, 34, 45],
    };
    println!("{}", my_scores);

    for perm in numbers.iter().permutations(11).unique() {
        // let word = Word{characters:perm};
        // println!("{}",word)
        println!("{:?}", perm);
    }

    // Create alphabet characters
    // let alphabet = (b'A'..=b'z')           // Start as u8
    // .map(|c| c as char)            // Convert all to chars
    // .filter(|c| c.is_alphabetic()) // Filter only alphabetic chars
    // .collect::<Vec<_>>();          // Collect as Vec<char>
    // println!("{:?}", alphabet);

    let characters = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    let n =minlen;
    println!("{:?}", minlen);
    let combinations : Vec<_> = (2..minlen).fold(
        characters.iter().cartesian_product(characters.iter()).map(|(&a, &b)| a.to_owned() + b).collect(),
        |acc, _| acc.into_iter().cartesian_product(characters.iter()).map(|(a, b)| a.to_owned() + b).collect()
    );
    println!("{:?}", combinations);
}
