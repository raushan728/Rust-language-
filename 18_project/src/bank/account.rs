#[derive(Debug)]
pub enum AccountType {
    Saving,
    Current,
}

pub fn create_account(name: &str, acc_type: AccountType) {
    println!("Creating account for: {}", name);
    println!("Account type: {:?}", acc_type);
}

pub fn show_balance(name: &str) {
    println!("Balance for {} is ₹10,000", name);
}
