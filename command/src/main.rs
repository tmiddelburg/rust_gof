use std::{rc::Rc, cell::RefCell};
use command::{BankController, Account, Deposit, Transfer, Withdrawal};
use rand::Rng;

fn main() {
    let mut controller = BankController::new();

    let account1 = Rc::new(RefCell::new(Account::new(String::from("Thomas"), random_account_number())));
    let account2 = Rc::new(RefCell::new(Account::new(String::from("Google"), random_account_number())));
    let account3 = Rc::new(RefCell::new(Account::new(String::from("Microsoft"), random_account_number())));

    controller.register(Box::new(Deposit{ account: Rc::clone(&account1), amount: 100000 }));
    controller.undo();
    controller.redo();

    controller.register(Box::new(Deposit{ account: Rc::clone(&account2), amount: 100000 }));
    controller.register(Box::new(Deposit{ account: Rc::clone(&account3), amount: 100000 }));
    controller.register(Box::new(Transfer{ from_account: Rc::clone(&account2), to_account: Rc::clone(&account1), amount: 50000 }));

    controller.undo();
    controller.undo();
    controller.redo();
    controller.redo();

    controller.register(Box::new(Withdrawal{ account: Rc::clone(&account1), amount: 150000 }));

    account1.as_ref().borrow_mut().clear_cache();
    account2.as_ref().borrow_mut().clear_cache();
    account3.as_ref().borrow_mut().clear_cache();

    controller.compute_balance();

    println!("Account1 has $ {}", account1.as_ref().borrow().balance_cache);
    println!("Account2 has $ {}", account2.as_ref().borrow().balance_cache);
    println!("Account3 has $ {}", account3.as_ref().borrow().balance_cache);
}

fn random_account_number() -> String {
    let num1 = rand::thread_rng().gen_range(0..999999);
    let num2 = rand::thread_rng().gen_range(0..999999);
    format!("{:0>6}{:0>6}", num1.to_string(), num2.to_string())
}
