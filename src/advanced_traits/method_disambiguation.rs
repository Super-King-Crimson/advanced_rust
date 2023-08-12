pub fn explain() {
    println!("Traits can have the same method name, and rust can call both methods");
    
    //you just need to let rust know which method you mean
    let kite = Kite {
        height: 10,
    };

    //the same
    kite.fly();
    Kite::fly(&kite);

    CanFly::fly(&kite);
    Wizard::fly(&kite);

    //this is alright, but what if you have an associated fn (static method) 
        //that doesn't take a self parameter?
        //You have to use fully qualified syntax
    Kite::shout_name();
    <Kite as Wizard>::shout_name();
}

pub struct Kite {
    height: u32
}

impl Kite {
    pub fn fly(&self) {
        println!("The kite flies like a kite! ({} meters high)", self.height);
    }

    pub fn shout_name() {
        println!("I AM KITE!");
    }
}

impl Wizard for Kite {
    fn fly(&self) {
        println!("The kite flies like a wizard!");
    }

    fn shout_name() {
        println!("MY NAME IS KITE THE WIZARD!!!!!!!!");
    }
}

impl CanFly for Kite {
    fn fly(&self) {
        println!("The kite flies!");
    }
}

pub trait Wizard {
    fn fly(&self);

    fn shout_name();
}

pub trait CanFly {
    fn fly(&self);
}