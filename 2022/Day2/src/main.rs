use std::fs::File;
use std::io::{BufReader, BufRead, Error};

use std::process::exit;

fn main() -> Result<(), Error> {
    let path = "src/input.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut score = 0;

    for line in buffered.lines() {
        let myline = line?.clone();  // moved variables workaround
        match myline.as_str() {
            "A Z" => {
                // shape selected: scissors (3 pts)
                // opponent: rock
                // loss (0 pts)
                score += 3 + 0;
            },
            "B Z" => {
                // shape selected: scissors (3 pts)
                // opponent: paper
                // win (6 pts)
                score += 3 + 6;
            },
            "C Z" => {
                // shape selected: scissors (3 pts)
                // opponent: scissors
                // draw (3 pts)
                score += 3 + 3;
            },


            "A Y" => {
                // shape selected: paper (2 pts)
                // opponent: rock
                // win (6 pts)
                score += 2 + 6;
            },
            "B Y" => {
                // shape selected: paper (2 pts)
                // opponent: paper
                // draw (3 pts)
                score += 2 + 3;
            },
            "C Y" => {
                // shape selected: paper (2 pts)
                // opponent: scissors
                // loss (0 pts)
                score += 2 + 0;
            },


            "A X" => {
                // shape selected: rock (1 pts)
                // opponent: rock
                // draw (3 pts)
                score += 1 + 3;
            },
            "B X" => {
                // shape selected: rock (1 pts)
                // opponent: paper
                // loss (0 pts)
                score += 1 + 0;
            },
            "C X" => {
                // shape selected: rock (1 pts)
                // opponent: scissors
                // win (6 pts)
                score += 1 + 6;
            },
            _ => {
                println!("Not an invalid input!");
                println!("Possibly the file from the AOC download has been tampered with.");
                exit(1);
            },
        }
    }

    println!("Score: {}", score);

    println!("\n\n\nPart 2");
    score = 0;  // reset

    let input_parttwo = File::open(path)?;  // we can't use the same input twice (moving variables)
    let buffered_parttwo = BufReader::new(input_parttwo);  // we can't use the same buffer twice

    for line in buffered_parttwo.lines() {
        let myline = line?.clone();  // moved variables workaround
        match myline.as_str() {
            "A X" => {
                // We need to lose; opponent chose rock
                // pick scissors (3 pts)
                // loss (0 pts)
                score += 3 + 0;
            },
            "B X" => {
                // We need to lose; opponent chose paper
                // pick rock (1 pts)
                // loss (0 pts)
                score += 1 + 0;
            },
            "C X" => {
                // We need to lose; opponent chose scissors
                // pick paper (2 pts)
                // loss (0 pts)
                score += 2 + 0;
            },

            "A Y" => {
                // We need to draw; opponent chose rock
                // pick rock (1 pts)
                // draw (3 pts)
                score += 1 + 3;
            },
            "B Y" => {
                // We need to draw; opponent chose paper
                // pick paper (2 pts)
                // draw (3 pts)
                score += 2 + 3;
            },
            "C Y" => {
                // We need to draw; opponent chose scissors
                // pick scissors (3 pts)
                // draw (3 pts)
                score += 3 + 3;
            },

            "A Z" => {
                // We need to win; opponent chose rock
                // pick paper (2 pts)
                // win (6 pts)
                score += 2 + 6;
            },
            "B Z" => {
                // We need to win; opponent chose paper
                // pick scissors (3 pts)
                // win (6 pts)
                score += 3 + 6;
            },
            "C Z" => {
                // We need to win; opponent chose scissors
                // pick rock (1 pts)
                // win (6 pts)
                score += 1 + 6;
            },
            _ => {
                println!("Not an invalid input!");
                println!("Possibly the file from the AOC download has been tampered with. (or there's a newline at the end)");
                exit(1);
            },
        }
    }

    println!("Score (part 2): {}", score);

    Ok(())
}
