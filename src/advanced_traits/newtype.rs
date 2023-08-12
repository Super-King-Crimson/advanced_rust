pub fn explain() {
    println!("Remember the orphan rule for traits?");

    //You can only impl a method on a trait if the trait or type the trait's being impl'd for
        //was made in our crate

    println!("It can be worked around with the newtype pattern");
    //All we have to do is make a struct that's simply a wrapper around the type we want to impl for
    //This pattern incurs no runtime performance: the wrapper is deleted at compile time

    for num in U8Wrapper(0) {
        print!("{num} ");

        if num % 20 == 0 {
            println!();
        }
    }

    //The issue with this pattern is that we don't have access to all methods on a u8 (like adding)
    //A couple solutions to this are:
        //Impl the deref trait on the newtype wrapper trait to return its contained value
        //Impl each method that we will use individually
}

struct U8Wrapper(u8);

impl Iterator for U8Wrapper {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 = match self.0.checked_add(1) {
            Some(val) => val,
            None => return None,
        };

        Some(self.0)
    }
} 