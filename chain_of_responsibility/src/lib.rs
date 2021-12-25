pub use supporter::{Supporter, Trouble};

mod supporter;

pub struct LimitedSupporter {
    name: String,
    next: Option<Box<dyn Supporter>>,
    limit_id: u32,
}

impl LimitedSupporter {
    pub fn new(name: String, next: Option<Box<dyn Supporter>>, limit_id: u32) -> LimitedSupporter {
        LimitedSupporter{ name, next, limit_id }
    }
}

impl Supporter for LimitedSupporter {
    fn name(&self) -> &String {
        &self.name
    }

    fn support(&self, trouble: supporter::Trouble) {
        if self.can_handle(&trouble) {
            self.supported(trouble);
        } else if let Some(s) = &self.next {
            s.support(trouble);
        } else {
            self.unsupported(trouble);
        }
    }

    fn can_handle(&self, trouble: &Trouble) -> bool {
        trouble.id <= self.limit_id
    }
}

pub struct MoodySupporter {
    name: String,
    next: Option<Box<dyn Supporter>>,
}

impl MoodySupporter {
    pub fn new(name: String, next: Option<Box<dyn Supporter>>) -> MoodySupporter {
        MoodySupporter{ name, next }
    }
}

impl Supporter for MoodySupporter {
    fn name(&self) -> &String {
        &self.name
    }

    fn support(&self, trouble: supporter::Trouble) {
        if self.can_handle(&trouble) {
            self.supported(trouble);
        } else if let Some(s) = &self.next {
            s.support(trouble);
        } else {
            self.unsupported(trouble);
        }
    }

    fn can_handle(&self, trouble: &Trouble) -> bool {
        trouble.id % 2 == 1
    }
}

pub struct SpecialSupporter {
    name: String,
    next: Option<Box<dyn Supporter>>,
    target_id: u32,
}

impl SpecialSupporter {
    pub fn new(name: String, next: Option<Box<dyn Supporter>>, target_id: u32) -> SpecialSupporter {
        SpecialSupporter{ name, next, target_id }
    }
}

impl Supporter for SpecialSupporter {
    fn name(&self) -> &String {
        &self.name
    }

    fn support(&self, trouble: supporter::Trouble) {
        if self.can_handle(&trouble) {
            self.supported(trouble);
        } else if let Some(s) = &self.next {
            s.support(trouble);
        } else {
            self.unsupported(trouble);
        }
    }

    fn can_handle(&self, trouble: &Trouble) -> bool {
        trouble.id == self.target_id
    }
}

pub struct LazySupporter {
    name: String,
    next: Option<Box<dyn Supporter>>,
}

impl LazySupporter {
    pub fn new(name: String, next: Option<Box<dyn Supporter>>) -> LazySupporter {
        LazySupporter{ name, next }
    }
}

impl Supporter for LazySupporter {
    fn name(&self) -> &String {
        &self.name
    }

    fn support(&self, trouble: supporter::Trouble) {
        if self.can_handle(&trouble) {
            self.supported(trouble);
        } else if let Some(s) = &self.next {
            s.support(trouble);
        } else {
            self.unsupported(trouble);
        }
    }

    fn can_handle(&self, trouble: &Trouble) -> bool {
        false
    }
}
