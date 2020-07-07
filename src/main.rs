use io::{stdin, stdout, Write};
use std::{io, process};

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choise = String::new();

    print!("Input a miner address: ");
    stdout().flush();
    stdin().read_line(&mut miner_addr);
    print!("Difficulty: ");
    stdout().flush();
    stdin().read_line(&mut difficulty);
    let diff = difficulty
        .trim()
        .parse::<u32>()
        .expect("We need an integer");
    println!("Generating genesis block! ");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);
    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine Block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        print!("Enter your choise: ");
        stdout().flush();
        choise.clear();
        stdin().read_line(&mut choise);
        println!("");

        match choise.trim().parse().unwrap() {
            0 => {
                println!("exiting...");
                process::exit(0);
            }
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("Enter sender address: ");
                stdout().flush();
                stdin().read_line(&mut sender);
                print!("Enter receiver address: ");
                stdout().flush();
                stdin().read_line(&mut receiver);
                print!("Enter amount: ");
                stdout().flush();
                stdin().read_line(&mut amount);

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );
                match res {
                    true => println!("Transaction added"),
                    false => println!("Transaction failed"),
                };
            }
            2 => {
                println!("Generating block...");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block genereated successfully"),
                    false => println!("Block generation failed"),
                };
            }
            3 => {
                let mut new_diff = String::new();
                print!("Enter new difficulty: ");
                stdout().flush();
                stdin().read_line(&mut new_diff);
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("Updated Difficulty"),
                    false => println!("Failed Updated Difficulty"),
                }
            }
            4 => {
                let mut new_reward = String::new();
                print!("Enter new reward: ");
                stdout().flush();
                stdin().read_line(&mut new_reward);
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Updated Reward"),
                    false => println!("Failed Updated Reward"),
                }
            }
            _ => println!("\tInvalid option please retry\t"),
        }
    }
}
