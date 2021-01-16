// use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead};
use rand::Rng;

fn getwords(count: usize, file: &str) -> std::io::Result<()>
{
    let mut words: Vec<String> = Vec::with_capacity(count);

    // get number of lines in file
    // let mut lines = 0;
    let input = File::open(file)?;
    let buffer = BufReader::new(input);
    let lines: Vec<_> = buffer.lines().collect();
    let maxline = lines.len() as u64;

    // for (index, sentence) in lines.into_iter().enumerate() {
    //
    //     println!("{}", index);
    // }

    for word in words.iter_mut() {
        let mut rng = rand::thread_rng();
        let rndln:u64 = rng.gen_range(1..maxline);

        for (index, sentence) in lines.iter().enumerate() {
            if index == (rndln as usize) {
                word = 
            }
        }

    }
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
    getwords(4, "/usr/share/dict/words");

}

