#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account { id, balance: 0, holder }
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
}

// first way to handle the owner ship model
// fn print_account(account:Account) -> Account {
//     println!("{:?}",account);
//     account
// }

fn print_account(account: &Account) {
    println!("{:?}",account);
}

fn main() {
    let bank = Bank::new();
    let mut account = Account::new(1, String::from("John"));

    println!("{:#?}", bank);

    // first way to handle the owner ship model
    // account = print_account(account); // now this is the owner of the account
    // account = print_account(account); // now this is the owner of the account
    // print_account(account);

    // second way to handle the owner ship model
    // by the passing the refernce of the value

    let account_ref = &account;

    print_account(account_ref);
    print_account(account_ref);
}
