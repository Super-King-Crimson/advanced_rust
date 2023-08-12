pub mod raw_pointers;
pub mod functions_and_methods;
pub mod mutable_static_variables;

pub static mut HI: i8 = 10;

pub fn explain() {
    println!("Unsafe Rust is a second hidden language that doesn't enforce Rust's guarantees");

    //Unsafe Rust exists because
        //Static analysis is, by nature, conservative
        //Underlying computer hardware is unsafe
    
    //to toggle an unsafe block, type unsafe with a block containing the unsafe code
    unsafe {
        println!("Within an unsafe block you can...");

        //Dereference raw pointers
        let mut a = String::from("Stringy!");
        a.as_mut_ptr().drop_in_place();

        //Call unsafe fns or methods
        add_ten_to_hi();

        //Interact with mutable static variables
        println!("{HI}");

        //Implement unsafe traits

        //Access fields of unions
        let union = Union { f1: 10 };
        println!("{}", union.f1);
    }

    //It's important to know that unsafe doesn't turn off the borrow checker or anything,
        //it just allows you to do things that aren't memory checked
    
    //Because unsafe blocks are the only places memory related bugs can happen, 
        //keep them small so they're easy to find
    //Also put unsafe code in a safe abstraction so it can be used safely

    //Now let's go over these superpowers individually
    raw_pointers::explain();
    functions_and_methods::explain();
    mutable_static_variables::explain();
}

unsafe fn add_ten_to_hi() {
    HI += 10;
}

union Union {
    f1: u32,
}