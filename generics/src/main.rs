mod basket;
mod stack;
mod container;

use num_traits::{Float, ToPrimitive};
use basket::Basket;
use stack::Stack;
use crate::container::Container;

trait Vehicle {
    fn start(&self);
    fn stop(&self) {
        println!("stopped");
    }
}

struct Car {}

impl Car {
    fn new() -> Self {
        Self {}
    }
}

impl Vehicle for Car {
    fn start(&self) {
        println!("Started");
    }
}

fn main() {
    generics();
    bskt();
    stk();
}

fn add_string<T: Container<String>>(c: &mut T, s: String) {
    c.put(s);
}

fn bskt() {
    let mut b1 = Basket::new(String::from("Hi there"));
    let b2 = Basket::new(10);
    let mut b3 = Basket::new(true);

    add_string(&mut b1, String::from("Aloha"));
    println!("{:#?}", b1);
    println!("{:#?}", b2);
    println!("{:#?}", b3);

}

fn stk() {
    let mut s1 = Stack::new(vec![String::from("Hi")]);
    let s2 = Stack::new(vec![1, 2, 3]);

    add_string(&mut s1, String::from("Aloha"));
    println!("{:#?}", s1);
    println!("{:#?}", s2);
}

fn generics() {
    let a_32: f32 = 3.0;
    let b_64: f64 = 4.0;
    let a_64 = a_32 as f64;
    let b_32 = b_64.to_f32().unwrap();

    println!("{}", solve(a_64, b_64));

    println!("{}", solve_fixed(a_32, b_32));
    println!("{}", solve_fixed(a_64, b_32));
    println!("{}", solve_fixed(a_32, b_64));
    println!("{}", solve_fixed(a_64, b_64));

    println!("{}", super_solve(a_32, b_32));
    println!("{}", super_solve(a_64, b_32));
    println!("{}", super_solve(a_32, b_64));
    println!("{}", super_solve(a_64, b_64));
    println!("{}", super_solve(a_64, 4));
    println!("{}", super_solve(3, b_64));

    let car = Car::new();
    car.start();
    car.stop();
}

fn solve(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}

fn solve_fixed<T: Float, U: Float>(a: T, b: U) -> f64 {
    let a_64 = a.to_f64().unwrap();
    let b_64 = b.to_f64().unwrap();
    (a_64.powi(2) + b_64.powi(2)).sqrt()
}

fn super_solve<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
    let a_64 = a.to_f64().unwrap();
    let b_64 = b.to_f64().unwrap();
    (a_64.powi(2) + b_64.powi(2)).sqrt()
}
