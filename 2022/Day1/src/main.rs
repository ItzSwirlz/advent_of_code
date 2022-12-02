use dict::{Dict, DictIface};

use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    // My first time reading from a file in Rust!
    // https://doc.rust-lang.org/book/ch12-02-reading-a-file.html is how you would read from a file
    // But here's what I'm going to do:
    //
    // I'm going to have each line represent a different Elf.
    // Every blank line hit is a new elf
    // Then I'll do something else I've never done in rust, dictionaries.
    // elf :: calories

    let mut elves = Dict::<String>::new();

    let path = "src/input.txt";
    let input = File::open(path)?;  // note to self: ? is error propagation - it can be an error
    let buffered = BufReader::new(input);

    let mut elf_sum = 0;  // to track the elf sums
    let mut current_key = 0;
    for line in buffered.lines() {
        let myline = line?.clone();  // moved variables workaround
        match myline.as_str() {
            _ => {
                if myline == "" {
                    elves.add(current_key.to_string(), elf_sum.to_string());
                    current_key += 1;  // different elf
                    elf_sum = 0;
                } else {
                    elf_sum += myline.parse::<u64>().unwrap();
                }
            }
        };
    }


    // Which elf has the most calories?
    let mut most_elf_calories_index = 0;  // index of dict it is in
    let mut best = 0;
    let mut test = 0;
    println!("For some reason, Rust was screwing with what the value thought it was larger or not.");
    println!("I have no idea why this happens, if someone can explain why to me I would appreciate it");
    println!("For now, take the highest of these values, which is the answer.\n\n");
    for i in &elves {
        if i.val.as_str() > elves.get(&most_elf_calories_index.to_string()).unwrap() {
            println!("Potential answer:");

            most_elf_calories_index = i.key.parse().unwrap();
            test = i.val.parse::<u64>().unwrap();
            if test > best {
                println!("{}", i.val.parse::<u64>().unwrap());
                println!("confirmed? read above message.\n");
            }
        }
    }
 
    println!("\n\nAnd the elf number/dictionary info I would adjust if I could figure out what was wrong with the above loop. Read code for details");
    println!("The elf with the most calories is elf number {}", most_elf_calories_index);
    println!("That elf has {:#?} calories", elves.get(&most_elf_calories_index.to_string()).unwrap());


    println!("\n\nPart 2");
    let mut part2_sum = 0;
    for i in &elves {
        // manual guess and check until 3 values
        // if i could sort the dictionary and get the last 3 values or base it off the highest i would
        // but both ideal solutions above have problems - can't sort dictionary (?) and rust can't find a highest
        if i.val.as_str() > "66000" {
            //println!("{:#?}", i.val.as_str())
            part2_sum += i.val.parse::<u64>().unwrap();
        }
    }
    part2_sum -= 8736;  // the weird extra one that sits in, above issues in Part 1 apply here
    println!("most top 3 elves carrying in total: {}", part2_sum);
    Ok(())
}