fn eat(_: Box<int>) { }

fn peek(b: &Box<int>) {
    println!("b -> {}", b);
}

fn main() {
    let i = 1i;
    let ri = &i;
    println!("{}", i);
    println!("{}", ri);

    let b = box 2i;
    peek(&b);
    eat(b);
}
