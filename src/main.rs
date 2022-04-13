// Written by Carson Woods
// 2022

use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

fn main() {
    // collect any numbers to test from arguments
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // this is the start point for calculations (save-state)
        // defaults to 2
        let mut number: u128;

        // scope ensures file is closed when handler is dropped
        {
            // opens record file if exists
            // creates blank file if not
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(".collatz")
                .expect("Error reading record file");

            // performs buffered read to get start number
            // as a string value
            let mut buf_reader = BufReader::new(file);
            let mut start = String::new();
            let _file_result = buf_reader.read_line(&mut start);

            // removes any leading or trailing whitespace
            start = start.split_whitespace().collect();

            // if file is not empty, parse into starting integer
            if start.chars().count() > 0 {
                match start.parse::<u128>() {
                    Ok(n) => {
                        // if number is valud
                        number = n;
                    }
                    Err(_) => {
                        // if file contains invalid number
                        println!("Error: non-integer argument found in record file");
                        std::process::exit(1)
                    }
                }
            } else {
                // if file is empty, set the start to 2
                number = 2;
            }
        }

        // infinite loop to perform calculations
        loop {
            let start = number.clone();

            // if number converges
            // for practical reasons, this will always converge
            // when using naive approach (no way to identify non-converging numbers)
            //if collatz_naive(number, false) {
            if collatz_benchmark_aware(number, start, false) {
                // if number is divisible by 100, write to file as checkpoint
                // reduces arbitrary file IO on every tested number
                if number % 100000 == 0 {
                    let mut file = BufWriter::new(
                        File::create(".collatz").expect("Unable to create checkpoint file"),
                    );
                    match write!(file, "{0}", number) {
                        Ok(_) => {
                            println!("Checkpoint saved for number {}", number);
                        }
                        Err(_) => {
                            // fails if argument passed is not integer
                            println!("Error: failed to write checkpoint");
                            std::process::exit(1)
                        }
                    }
                }
            } else {
                // indicate that a number does not converge
                println!("Found non converging number: {}", number);
                std::process::exit(1)
            }
            // increment number to test
            number = number + 1;
        }
    } else {
        // this runs for any number specified to be tested manually
        for arg in args[1..].iter() {
            // or, to be safe, match the `Err`
            match arg.parse::<u128>() {
                Ok(n) => {
                    collatz_naive(n, true);
                }
                Err(_) => {
                    // fails if argument passed is not integer
                    println!("Error: non-integer argument passed");
                    std::process::exit(1)
                }
            }
        }
    }
}

// solves collatz conject for a single number
// returns true if converges, false if never converges
fn collatz_naive(mut number: u128, print: bool) -> bool {
    // clones original number for record purposes
    let orig_num: u128 = number.clone();

    // counts number of iterations
    // starting at 1 includes end state
    let mut iter: u128 = 1;

    // actual collatz conjecture algorithm
    // see: https://en.wikipedia.org/wiki/Collatz_conjecture#Statement_of_the_problem
    while number != 1 {
        iter = iter + 1;
        if number % 2 == 0 {
            // number is even
            number = number / 2;
        } else {
            // number is odd
            // extra logic is necessary to handle possible
            // overflow induced incorrectness
            number = match number.checked_mul(3) {
                Some(n) => n,
                None => {
                    println!("Error: multiplication overflow");
                    std::process::exit(1)
                }
            };

            number = match number.checked_add(1) {
                Some(n) => n,
                None => {
                    println!("Error: addition overflow");
                    std::process::exit(1)
                }
            };
        }
    }

    if print {
        println!(
            "Number: {} converges to 1 after {} iterations!",
            orig_num, iter
        );
    }
    return true;
}

// solves collatz conject for a single number
// returns true if converges, false if never converges
fn collatz_benchmark_aware(mut number: u128, start: u128, print: bool) -> bool {
    // clones original number for record purposes
    let orig_num: u128 = number.clone();

    // counts number of iterations
    // starting at 1 includes end state
    let mut iter: u128 = 1;

    // collatz conjecture algorithm
    // with simplification:
    //    any number found to be smaller
    //    than previously checked value must
    //    mean the number converges
    while number != 1 {
        iter = iter + 1;
        if number <= start {
            return true;
        }

        if number % 2 == 0 {
            // number is even
            number = number / 2;
        } else {
            // number is odd
            // extra logic is necessary to handle possible
            // overflow induced incorrectness
            number = match number.checked_mul(3) {
                Some(n) => n,
                None => {
                    println!("Error: multiplication overflow");
                    std::process::exit(1)
                }
            };

            number = match number.checked_add(1) {
                Some(n) => n,
                None => {
                    println!("Error: addition overflow");
                    std::process::exit(1)
                }
            };
        }
    }

    if print {
        println!(
            "Number: {} converges to 1 after {} iterations!",
            orig_num, iter
        );
    }
    return true;
}
