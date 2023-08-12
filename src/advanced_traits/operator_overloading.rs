pub fn explain() {
    println!("Generics have defaults that can be specified with <PlaceholderT=ConcreteT>");

    //Let's say we want to do some good old operator overloading
    let (point_ones, pt) = (Vector3::new(0.1, 0.1, 0.1), Vector3::new(3.2, -10.0, 0.0001));
    //unfortunately rust doesn't allow you to create our own operators or overload random operators,
        //but we can overload operations in std::ops by implementing their traits

    println!("point_ones * pt =  {:?}", point_ones * pt);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }
}

use std::ops::Mul;
impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

//Note that we didn't have to specify any default type params for the Sub impl
//we specified a default type for Rhs (right hand side), so the default is SelfType * SelfType
trait _Mul<Rhs=Self> {
    type Output;

    fn mul(self, rhs: Rhs) -> Self::Output;
}

//But we can overload this to accept more operators (like Vector3 * f32)
impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}


//Default type parameters are used in two main ways:
    //To extend a type without breaking existing code
    //To allow customization in specific cases most users won’t need (like adding two unlike types)
//This gets rid of implementation boilerplate for a trait, making it easier to use