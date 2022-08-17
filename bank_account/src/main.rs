use std::io;
use rand::Rng;

const CHARSET: &[u8] = b"0123456789";
const ACCOUNT_NUMBER_LENGTH: usize = 10;
const MINIMUM_BALANCE: i32= 1000;

struct BankUser {
    account_name: String,
    account_number: u8,
    account_balance: f64
}

fn main() {
    front_desk();

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("You have given me a wrong input you nutjob.");

        let input: char = input.trim().to_uppercase().parse().expect("Nigga! I said I want a fucking character.");

        let current_user = BankUser {
            account_name: String::new(),
            account_balance: 0.0,
            account_number: 0
        };

        match input {
            'A' => handle_new_account(current_user),
            'B' => handle_account_balance_display(current_user),
            'C' => handle_deposit(current_user),
            'D' => handle_withdrawal(current_user),
            'E' => {
                println!("Thank you for banking with us!");
                break;
            },
            _ => {
                println!("You piece of shit!! The fuck is this? {}. This isn't what I asked for!! Try again, you poor excuse for a meat bag!", input);
                continue;
            }
        };

        print!("$: ");
    }
}

fn front_desk() {
    println!("=================================================================");
    println!("Welcome to City of a Thousand Planets Bank.");
    println!("I am Nanda! What would you like to do today?");

    println!("A. Create new account.");
    println!("B. Display Balance amount.");
    println!("C. Deposit money.");
    println!("D. Withdraw money.");
    println!("E. Exit.");

    println!();
    println!("Select your choice: ");

    println!("=================================================================");
}

fn handle_new_account(mut current_user: BankUser) -> BankUser {

    let mut account_name = String::new();
    let mut amount = String::new();

    println!("Please enter your name. This would be your account name.");
    io::stdin()
        .read_line(&mut account_name)
        .expect("You have given me a wrong input you nutjob.");

    println!("Enter an opening amount. Minimum is {} ", MINIMUM_BALANCE);
    io::stdin()
        .read_line(&mut amount)
        .expect("You have given me a wrong input you nutjob.");

    let amount: f64 = amount.trim().parse().expect("Nigga! I said I want a fucking number.");

    current_user.account_name = account_name;
    current_user.account_number = generate_account_number();
    current_user.account_balance = amount;

    println!("Congratulations. Your new account has been created!!");
    println!("------------------------------------------------------");

    let current_user = handle_account_balance_display(current_user);

    current_user
}

fn handle_account_balance_display(current_user: BankUser) -> BankUser {

    println!("Account Name: {} ", current_user.account_name);
    println!("Account Number: {} ", current_user.account_number);
    println!("Balance: {} ", current_user.account_balance);

    current_user
}

fn generate_account_number() -> u8 {
    
    let mut range = rand::thread_rng();

    let account_number: String = (0..ACCOUNT_NUMBER_LENGTH)
        .map(|_| {
          
            let idx = range.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("{:?}", account_number);

    let account_number: u8 = match account_number.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    account_number
}

fn handle_deposit(mut current_user: BankUser) -> BankUser {

    let mut deposit_amount = String::new();

    println!("Enter an amount to deposit.");
    io::stdin()
        .read_line(&mut deposit_amount)
        .expect("You have given me a wrong input you nutjob.");

    let deposit_amount: f64 = deposit_amount.trim().parse().expect("Nigga! I said I want a fucking number.");

    current_user.account_balance += deposit_amount;

    println!("------------------------------------------------------");
    println!("Balance: {} ", current_user.account_balance);

    current_user
}

fn handle_withdrawal(mut current_user: BankUser) -> BankUser {
    current_user
}