// use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::clone;
use censor::Censor;
use rand::{
    Rng,
    distributions::{Distribution, Uniform},
};


fn validate(word: &String) -> bool
{
    let censor = Censor::Standard;
    if word.len() < 4 {
        return false;
    } else if *word.to_uppercase() == *word || word.contains("'") || word == "wheres" {
        return false;
    } else {
        return !censor.check(word);
    }
}

fn getwords(count: usize, file: &str) -> std::io::Result<()>
{
    let mut words = vec!["".to_string(); count];

    // get number of lines in file
    // let mut lines = 0;
    let input = File::open(file)?;
    let buffer = BufReader::new(input);
    let mut lines: Vec<_> = buffer.lines().map(|l| l.expect("Could not read a line")).collect();
    let maxline = lines.len() as u64;

    println!("{}", "How about here?");
    println!("{}", words.len());


    for indx in 0..words.len() {

        'outer: loop {
            let mut rng = rand::thread_rng();
            let die_range = Uniform::new_inclusive(1, maxline);
            let die = die_range.sample(&mut rng);

            println!("{}", "Also Here");

            for (index, text) in lines.iter_mut().enumerate() {
                // println!("{}", "Am Here");
                if index == (die as usize) {
                    if validate(text) {
                        let ch = text.to_lowercase().chars().nth(0);
                        if words.iter().any(|i| i.to_lowercase().chars().nth(0) == ch) {
                            // println!("ch: {:?}", ch);
                            // println!("words: {:?}", words);
                            // println!("{}", "uh oh");
                            continue 'outer;
                        }
                        // print!("{:?}", c);
                        // words.iter().any(|i| i.chars().nth(0) == ch);
                        words[indx] = text.clone();
                        println!("{}: {}", "Okay", text);
                        break 'outer;
                    }
                    continue 'outer;
                }
            }
        }
    }
    println!("{:?}", words);
    // let mut rng = rand::thread_rng();
    // let rndln:u64 = rng.gen_range(1..lines);
    // println!("{}", rndln);
    Ok(())
}

fn main()
{
    // let mut rng = rand::thread_rng();
    // let rndln:u32 = rng.gen_range(1,
    // println!("{}", rndln);
    getwords(6, "/usr/share/dict/words");

}

