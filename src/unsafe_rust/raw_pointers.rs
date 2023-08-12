pub fn explain() {
    //Unsafe Rust has raw pointers, that are like references: *const T for immut and *mut T for mutable

    //However, they aren't borrow checked, so we can have multiple mutable refs
    println!("Raw pointers aren't guaranteed to point to valid memory, so they can be null");
    //They also aren't auto-cleaned

    let mut num = 5;

    //here are some raw pointers, made from refs
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    //we can create them, but we can't deref them unless we're in an unsafe block

    //Here's a raw pointer that we don't know is valid or not: points to some location in memory
    let address = 0x012345usize;
    let _r = address as *const i32;

    //Nothing's wrong with creating raw pointers, but when we try to deref we might get an invalid value
    unsafe {
        println!("r1: {r1:?} -> {a}, r2: {r2:?} -> {b}", a = *r1, b = *r2);
    }
    //oh also notice that there's no problem with having mutable and immutable raw pointers at the same time
        //That means raw pointers can cause data races

    //However, raw pointers have a use-case in interfacing w/C code and calling unsafe functions or methods
}
