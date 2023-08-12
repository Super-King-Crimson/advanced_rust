pub fn explain() {
    println!("Good luck finding an unsafe trait to impl lmao");
    
    //Unsafe traits have at least one method with an invariant the compiler can't verify
    //An example of unsafe traits are Sync and Send: if your type has stuff that can't be verified as Sync and Send
        //like raw pointers
    //then you have to manually ensure that the type is safe to Sync and Send, then implement the traits yourself
}

pub unsafe trait Dangerous {}

unsafe impl Dangerous for i64 {}