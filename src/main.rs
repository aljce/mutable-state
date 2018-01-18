use std::cell::Cell;

struct A {
    i: Cell<u8>
}

impl A {
    fn new(i: u8) -> A {
        A { i: Cell::new(i) }
    }
}

impl PartialEq<u8> for A {
    fn eq(&self, other: &u8) -> bool {
        let val = self.i.get();
        let res = val == *other;
        self.i.set(val + 1);
        res
    }
}

fn main() {
    let a = A::new(1);
    if a == 1 && a == 2 && a == 3 {
        println!("!");
    }
}
