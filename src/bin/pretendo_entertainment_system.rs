#![feature(generators, generator_trait)]
use pretendo_entertainment_system;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

trait Opcode<'a> {
    fn generate(val: &'a RefCell<Bop>) -> Pin<Box<dyn Generator<Yield = Bop, Return = Bop> + 'a>>;
}

struct AbsoluteX;

#[derive(Debug, Clone)]
struct Bop(u8);

impl<'a> Opcode<'a> for AbsoluteX {
    fn generate(val: &'a RefCell<Bop>) -> Pin<Box<dyn Generator<Yield = Bop, Return = Bop> + 'a>> {
        Box::pin(move || {
            yield val.borrow().clone();
            val.borrow_mut().0 += 1;
            val.borrow().clone()
        })
    }
}

fn main() {
    let bop = RefCell::new(Bop(0));
    let mut generator = AbsoluteX::generate(&bop);
    if let GeneratorState::Yielded(x) = generator.as_mut().resume(()) {
        println!("{:?}", bop);
        println!("{:?}", x);
    }
    let mut gen = AbsoluteX::generate(&bop);
    if let GeneratorState::Yielded(x) = gen.as_mut().resume(()) {
        println!("{:?}", bop);
        println!("{:?}", x);
    }
    generator.as_mut().resume(());
}
