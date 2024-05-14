fn create_account(has_account: bool) {
    if has_account == true {
        println!("Creating Account ...!!!");
    } else {
        println!("Account already exists ...!!!");
    }
}

fn main() {
  create_account(true);
}
