pub fn explain() {
    //Keyword extern allows fn signatures to be declared that may map to other languages
    //obviously since those fns can't be guaranteed safe by rustc, extern fn calls are always unsafe
    unsafe { 
        println!("The absolute value of -11 (according to C) is... {}", abs(-11));
    }
    
    //The 'C' string has the names and signatures of external functions from the application binary interface (ABI)
    //ABI is just how the function's called at the assembly level: arg types and return type
}

extern "C" {
    //ok apparently only C is cool enough for rust :sob:
    fn abs(input: i32) -> i32;
}

//we can even make fns that are callable from C
//we have to make sure the compiler doesn't change this name by saying #[no-mangle]
#[no_mangle]
pub extern "C" fn call_from_c() {
    //mangling is when a compiler changes the name of a fn so it has more info for other parts of compilation
    //but the name is kinda unreadable for humans so
    //no mangle.

    println!("Hello, C");
}