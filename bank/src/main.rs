use std::fmt::Debug;

#[derive(Debug)]
struct Bank {
    name: String,
    accounts: Vec<Account>,
}

#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Bank {
    fn new(name: String) -> Self {
        Self { name, accounts: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn balance(&mut self) -> i32 {
        self.accounts.iter().map(|a| a.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts.iter().map(|a| a.summary())
            .collect::<Vec<String>>()
    }
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Self { id, balance: 0, holder }
    }

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }

    fn summary(&self) -> String {
        format!("{} has ${}", self.holder, self.balance)
    }
}

fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn main() {
    let mut bank = Bank::new(String::from("Allianz"));
    let mut account = Account::new(1, String::from("Anatoli Vladev"));
    account.deposit(25000);
    account.withdraw(1);
    println!("{}", account.summary());
    print_account(&account);
    bank.add_account(account);
    println!("{:?}", bank.balance());
    println!("{:?}", bank.summary());
}
