use std::{rc::Rc, cell::RefCell};

pub trait Transaction {
    fn execute(&mut self);
}

pub struct BankController {
    ledger: Vec<Box<dyn Transaction>>,
    current: usize,
}

impl BankController {
    // TODO make a default?
    pub fn new() -> BankController {
        BankController { ledger: Vec::new(), current: 0 }
    }

    pub fn register(&mut self, transaction: Box<dyn Transaction>) {
        self.ledger.drain(self.current..self.ledger.len());
        self.ledger.push(transaction);
        self.current += 1;
    }

    pub fn undo(&mut self) {
        if self.current > 0 {
            self.current -= 1;
        }
    }

    pub fn redo(&mut self) {
        if self.current < self.ledger.len() {
            self.current += 1;
        }
    }

    pub fn compute_balance(&mut self) {
        for transaction in &mut self.ledger[..self.current] {
            transaction.execute();
        }
    }
}

pub struct Account {
    name: String,
    number: String,
    pub balance_cache: u32,
}

impl Account {
    pub fn new(name: String, number: String) -> Account {
        Account { name, number, balance_cache: 0 }
    }

    fn deposit(&mut self, amount: u32) {
        self.balance_cache += amount;
    }

    fn withdraw(&mut self, amount: u32) {
        if amount > self.balance_cache {
            panic!("Insufficient funds!")
        }
        self.balance_cache -= amount;
    }
    
    pub fn clear_cache(&mut self) {
        self.balance_cache = 0;
    }
}

pub struct Deposit {
    pub account: Rc<RefCell<Account>>,
    pub amount: u32,
}

impl Deposit {
    fn transfer_details(&self) -> String {
        format!("{} to account {}", self.amount / 100, self.account.as_ref().borrow().name)
    }
}

impl Transaction for Deposit {
    fn execute(&mut self) {
        self.account.as_ref().borrow_mut().deposit(self.amount);
        println!("Deposited {}", self.transfer_details());
    }
}

pub struct Withdrawal {
    pub account: Rc<RefCell<Account>>,
    pub amount: u32,
}

impl Withdrawal {
    fn transfer_details(&self) -> String {
        format!("{} from account {}", self.amount / 100, self.account.as_ref().borrow().name)
    }
}

impl Transaction for Withdrawal {
    fn execute(&mut self) {
        self.account.as_ref().borrow_mut().withdraw(self.amount);
        println!("Withdrawn {}", self.transfer_details());
    }
}

pub struct Transfer {
    pub from_account: Rc<RefCell<Account>>,
    pub to_account: Rc<RefCell<Account>>,
    pub amount: u32,
}

impl Transfer {
    fn transfer_details(&self) -> String {
        format!("{} from account {}\nto account {}", self.amount / 100, self.from_account.as_ref().borrow().name, self.to_account.as_ref().borrow().name)
    }
}

impl Transaction for Transfer {
    fn execute(&mut self) {
        self.from_account.as_ref().borrow_mut().withdraw(self.amount);
        self.to_account.as_ref().borrow_mut().deposit(self.amount);
        println!("Transferred {}", self.transfer_details())
    }
}
