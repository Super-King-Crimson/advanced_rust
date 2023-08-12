use std::ops::Mul;

pub fn explain() {
    println!("Supertraits are a trait that require another trait to be implemented to use");

    //Let's make a Square trait: for it to be impl'd, the type must be able to be multiplied
    let a = 10.square();
    println!("{a}");
}

impl Square for i32 {
    fn square(self) -> Self {
        self * self
    }
}

//this won't work: counter doesn't impl Mul, Copy, or Clone
// impl Square for super::associated_types::Counter { }

//For a type to be squared, it must have- oh dang
pub trait Square: Mul + Sized + Copy + Clone {
    fn square(self) -> Self;
}