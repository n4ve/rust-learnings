use std::io;

fn main() {
    loop {
        println!("Please Enter n where n is equals or less than 10,000.  Type \'exit\' to exit program");
        let mut input = String::new();
        let stdin = io::stdin();

        stdin.read_line(&mut input).expect("Input failed");
        let result: String = input.trim().parse().expect("Parse failed");

        if result == "exit" {
            break;
        }

        match result.trim().parse::<i32>() {
            Ok(num) => {
                match num {
                    0..= 10_000 => {
                        println!("n is {}", num);
                        fizz_buzz(num);
                    }
                    i => {
                        println!("n is out of range : {}", i);
                    }
                }
            }
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
            }
        }
    }
}

fn fizz_buzz(n: i32){
    const FIZZ: &str = "Fizz";
    const BUZZ: &str = "Buzz";
    const FIZZBUZZ: &str = "FizzBuzz";

    let mut result: Vec<String> = Vec::new();

    for i in 1..n+1 {
        let is_fizz = i % 3 == 0;
        let is_buzz = i % 5 == 0;
        if is_fizz && is_buzz {
            result.push(FIZZBUZZ.to_string());
        } else if is_fizz {
            result.push(FIZZ.to_string());
        } else if is_buzz {
            result.push(BUZZ.to_string());
        } else {
            result.push(i.to_string());
        }
    }

    let result_joined = result.join(", ");

    println!("{}", result_joined);
}