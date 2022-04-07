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
        None => println!("No idea what your favorite number is."),
        Some(s) => {
            match s.parse::<usize>() {
                Ok(n) => minlen =n,
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

    for perm in numbers.iter().permutations(minlen).unique() {
        // let word = Word{characters:perm};
        // println!("{}",word)
        println!("{:?}", perm);
    }
}
