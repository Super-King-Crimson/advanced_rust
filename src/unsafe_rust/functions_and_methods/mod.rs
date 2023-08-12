pub mod externals;

use std::slice;

pub fn explain() {
    println!("Unsafe fns are just normal functions with unsafe in them");

    //They indicate that there are requirements Rust can't guarantee we've upheld to call this fucntion
        //This is one of the 'static analysis fails' things: compiler can't verify safety or unsafety,
        //so its up to the programmer to stay safe by knowing what the function must do

    unsafe {
        unsafe_fn()
    };
    
    //An unsafe fn can be wrapped in a safe abstraction that will guarantee the unsafe guarantees are upheld
        //For example, let's look at split_at_mut 
    let mut vec = vec![
        "Hello, world!".to_string(),
        "FooBarBazQux".to_string(),
        "0123456789".to_string(),
        "xyz".to_string(),
        "abc".to_string(),
    ];

    //in case you didn't know, split_at_mut allows you to do things like this:
    let slice = &mut vec[..];
    
    let (first, second) = slice.split_at_mut(2);
    
    //hey, two mutable references to an array! i thought the compiler didn't let you do that!
    first[1].push_str(" (please use lower_snake_case)");
    second.last_mut().unwrap().push_str("def");
    
    //Let's look at split_at_mut's definition:
    //split_at_mut();

    //then let's talk about calling (or getting called by) functions from other languagess
    externals::explain();
}

unsafe fn unsafe_fn() {
    //remember, this is that public static mutable var: the bodies of unsafe fns are unsafe blocks
    super::HI = 0;
}

fn _split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    //to guarantee that BOTH of these pointers are valid...
    let len = values.len();
    let ptr = values.as_mut_ptr();
    
    //the 'middle' of the slice MUST be smaller or equal to the length of the slice
    assert!(mid <= len);

    //hey, we wrapped around some unsafe code! 
        //But because we verified that we aren't accessing data beyond the length of the slice, we're ok
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),

            //we also know this pointer's pointing to valid data, because mid is within the slice
            //even if mid == len, then len - mid = 0 and 0 items will be returned from beyond the slice len
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}