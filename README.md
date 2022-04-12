# Collatz

This is a rust-based tester for checking if numbers adhere to the [Collatz Conjecture](https://en.wikipedia.org/wiki/Collatz_conjecture).

## Getting Started

You only need rust installed. There are no dependencies. 

## How to use

##### All numbers

To start checking any number starting from 2 going to infinity (well more specifically to the limit of u128), run:
```
cargo run --release
```
This will create a `.collatz` file which will store checkpoints. If this file is not deleted, it will store the latest number it has checked which was divisible by 100,000. 
Given that no counter-example to the conjecture exists for 2^68, only the most recent checkpoint value is stored. 

##### Specific numbers

To check specific numbers, run:
```
cargo run --release [numbers]
```
This will check any integer numbers that you choose. These numbers **must** be positive. 
An example of this type of launch is:
```
cargo run --release 12 36 19
```
which will result in the output:
```
Number: 12 converges to 1 after 10 iterations!
Number: 36 converges to 1 after 22 iterations!
Number: 19 converges to 1 after 21 iterations!
```

## FAQ
(if FAQ means frequently-(not)-asked questions)

#### Q. Why did you write this? 

Primarily because I wanted to have more experience learning Rust and also this seems like a fun problem with very low technical overhead, 
but with lots of opportunity for both improvements to how it runs (arguments, checkpointing, etc.) and improvements to the implementation.

#### Q. Why Rust?

The Collatz conjecture has an element of randomness to it certain numbers are time consumimng to check (especially naively). 
With that in mind, I wanted to choose a performant language. 
While I *could* have written this in C or C++, there is a reasonable argument to be made that new code shouldn't be written in unsafe languages. 
That almost certainly is **NOT** important for this program, but I want experience in Rust for other projects, so this works for that.
Plus, much of Rust's performance is not out-of-the-box, so I wanted an opportunity to practice optimizing Rust programs. 
This seems like a reasonable sandbox to do that. 
