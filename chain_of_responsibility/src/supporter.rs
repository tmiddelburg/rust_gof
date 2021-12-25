use std::fmt;

pub struct Trouble {
    pub id: u32,
}

impl fmt::Display for Trouble {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

pub trait Supporter {
    fn name(&self) -> &String;
    fn support(&self, trouble: Trouble);
    fn can_handle(&self, trouble: &Trouble) -> bool;

    fn supported(&self, trouble: Trouble) {
        println!("{} was handled by {}", trouble, self.name());
    }

    fn unsupported(&self, trouble: Trouble) {
        println!("{} was not handled.", trouble);
    }
}
