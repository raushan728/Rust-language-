use project::bank::account::{create_account, show_balance, AccountType};

fn main() {
    create_account("Raushan", AccountType::Saving);
    show_balance("Raushan");
}
