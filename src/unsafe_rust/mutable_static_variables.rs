pub static INFO: [&str; 5] = [
    "Let's talk about static variables, aka rust global variables",
    "They haven't been talked about until now because they don't play nice with rust's ownership rules",
    "For example, if two threads access a mutable static variable at the same time, a data race can occur",
    "Statics are like consts: always in SCREAMING_SNAKE_CASE and can only store references with 'static lifetimes",
    "However, a small difference is that statics will always be in the same location in memory, and can be mutable",
];
static mut LIKE_THIS_ONE: i32 = 10;

pub fn explain() {
    //Accessing an immutable static variable is like accessing a constant, and is therefore safe
    print!("\n");
    for msg in INFO {
        println!("{msg}");
    }
    println!();

    //interaction with ANY static mut is unsafe
    unsafe {
        LIKE_THIS_ONE -= 9;
    }

    //Don't use global static mut variables unless you have to: 
        //it's very hard to guarantee no data races if you use concurrency
}