#![feature(generators, generator_trait)]
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;
mod address;
mod bitops;
mod cpu;

trait Opcode {
    fn generate() -> Pin<Box<dyn Generator<Yield = String, Return = ()>>>;
}

struct AbsoluteX;

impl Opcode for AbsoluteX {
    fn generate() -> Pin<Box<dyn Generator<Yield = String, Return = ()>>> {
        Box::pin(move || {
            yield String::from("hello world!");
        })
    }
}

fn main() {
    let mut generator = AbsoluteX::generate();
    if let GeneratorState::Yielded(x) = generator.as_mut().resume(()) {
        println!("{}", x);
    }
}
