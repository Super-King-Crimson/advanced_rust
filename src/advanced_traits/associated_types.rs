pub fn explain() {
    println!("Associated types connect a type with a trait");

    //The impl of a trait will specify the concrete type
    //best example is an iterator
    let counter = Counter { count: 0, max_count: 10 };
    for num in counter {
        print!("{num}");
    }
}

//Iterator trait is defined as
pub trait _Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

//Associated types kinda seem like generics
pub struct Counter {
    count: u32,
    max_count: u32,
}

//so why couldn't we just use that?
/*
pub trait GenericIterator<T> {
    fn next(&mut self) -> Option<T>;
}

impl<T> GenericIterator<T> for Counter {
    fn next(&mut self) -> Option<T> {
        self.count += 1;
        Some(self.count)   <---- Wait, this isn't of type T! ...what is T, anyways?
    }
}
*/

//When we impl for a generic trait, we have to annotate our types (so a u32 here)
//but that means we  could impl<String> for Counter or impl<bool> or anything else (which makes no sense)

//For associated types, annotations aren't necessary: we only need to impl Iterator ONCE for counter

impl Iterator for Counter {
    //now iterator has only one meaning for counter
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max_count {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}