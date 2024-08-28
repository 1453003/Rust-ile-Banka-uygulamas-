use std::io;


trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!("Deposited ${} into account {}", amount, self.account_number);
        } else {
            println!("Deposit amount must be positive.");
        }
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount > 0.0 {
            if amount <= self.balance {
                self.balance -= amount;
                println!("Withdrew ${} from account {}", amount, self.account_number);
                Ok(())
            } else {
                Err("Insufficient funds".to_string())
            }
        } else {
            Err("Withdrawal amount must be positive".to_string())
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut input = String::new();


    println!("Enter the account number for Account 1:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let account_number1 = input.trim().to_string();
    input.clear();

    println!("Enter the holder name for Account 1:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let holder_name1 = input.trim().to_string();
    input.clear();

    println!("Enter the initial balance for Account 1:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let balance1: f64 = input.trim().parse().expect("Invalid balance input");
    input.clear();

    let mut account1 = BankAccount {
        account_number: account_number1,
        holder_name: holder_name1,
        balance: balance1,
    };

   
    println!("Enter the account number for Account 2:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let account_number2 = input.trim().to_string();
    input.clear();

    println!("Enter the holder name for Account 2:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let holder_name2 = input.trim().to_string();
    input.clear();

    println!("Enter the initial balance for Account 2:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let balance2: f64 = input.trim().parse().expect("Invalid balance input");
    input.clear();

    let mut account2 = BankAccount {
        account_number: account_number2,
        holder_name: holder_name2,
        balance: balance2,
    };

 
    println!("Enter the deposit amount for Account 1:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let deposit_amount: f64 = input.trim().parse().expect("Invalid deposit amount");
    input.clear();
    account1.deposit(deposit_amount);

    println!("Enter the withdrawal amount for Account 2:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let withdraw_amount: f64 = input.trim().parse().expect("Invalid withdrawal amount");
    input.clear();
    match account2.withdraw(withdraw_amount) {
        Ok(_) => println!("Withdrawal successful."),
        Err(e) => println!("Error: {}", e),
    }

    println!("Account {} balance: ${}", account1.account_number, account1.balance());
    println!("Account {} balance: ${}", account2.account_number, account2.balance());
}
