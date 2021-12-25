use chain_of_responsibility::{LimitedSupporter, SpecialSupporter, MoodySupporter, LazySupporter, Supporter, Trouble};

fn main() {
    let joseph = LimitedSupporter::new(String::from("Joseph"), None, 5);
    let amelia = SpecialSupporter::new(String::from("Amelia"), Some(Box::new(joseph)), 6);
    let william = MoodySupporter::new(String::from("William"), Some(Box::new(amelia)));
    let emily = LazySupporter::new(String::from("Emily"), Some(Box::new(william)));

    for i in 0..10 {
        emily.support(Trouble{id: i});
    }
}
