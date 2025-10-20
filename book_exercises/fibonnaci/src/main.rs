use std::io;

fn main() {
    println!("Fibonnaci!\n");
    println!("This program will print the n-th Fibonacci number.");

    let fib_idx = loop {

        println!("Please input which Fibonnaci number you want to see.");
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Could not read input.");

        match input.trim().parse::<u64>() {
            Ok(num) => break num,
            Err(_) => println!("Please enter a number.")
        }

    };
    let idx_suffix = match fib_idx{
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th"
    };
    println!("You want to see the {:?}{idx_suffix} Fibonnaci number.", fib_idx);

    let mut l: u128 = 0;
    let mut m: u128 = 1;
    let mut n: u128;


    for _i in 1..=fib_idx {
        n = l + m;
        m = l;
        l = n;
    }

    println!("Fibonnaci nr {fib_idx} = {l}!");
}
