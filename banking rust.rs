// Step 1: Create a Trait called Account
trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}

// Step 2: Implement the Account Trait for a struct called BankAccount
struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

// Step 3: Implementation of deposit method
impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    // Step 4: Implementation of withdraw method
    fn withdraw(&mut self, amount: f64) {
        if amount <= self.balance {
            self.balance -= amount;
        } else {
            println!("Insufficient funds!");
        }
    }

    // Step 5: Implementation of balance method
    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    // Step 6: Create two BankAccount instances with different account numbers and holder names
    let mut account1 = BankAccount {
        account_number: 1,
        holder_name: String::from("John Doe"),
        balance: 1000.0,
    };

    let mut account2 = BankAccount {
        account_number: 2,
        holder_name: String::from("Jane Smith"),
        balance: 1500.0,
    };

    // Step 7: Call the deposit method on one of the accounts
    account1.deposit(500.0);

    // Step 8: Call the withdraw method on the other account
    account2.withdraw(200.0);

    // Step 9: Call the balance method on both accounts and print the result
    println!("Account 1 Balance: ${}", account1.balance());
    println!("Account 2 Balance: ${}", account2.balance());
}
