#[derive(Debug, Clone)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            balance: 0,
            holder,
        }
    }

    fn deposite(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }

    fn summary(&self) -> String {
        format!("{} balance is {}", self.holder, self.balance)
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }
}

// first way to handle the owner ship model
// fn print_account(account:Account) -> Account {
//     println!("{:?}",account);
//     account
// }

// fn make_and_print_account(account: &Account) {
//     let account = Account::new(1, String::from("First"));
//     println!("{:?}",account);
// }

fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("John"));

    account.deposite(500);
    account.withdraw(200);

    bank.add_account(account);

    println!("{:#?}", bank);
    println!("{}", bank.total_balance());

    // first way to handle the owner ship model
    // account = print_account(account); // now this is the owner of the account
    // account = print_account(account); // now this is the owner of the account
    // print_account(account);

    // second way to handle the owner ship model
    // by the passing the refernce of the value

    // let account_ref = &account;

    // print_account(account_ref);
    // print_account(account_ref);
}
