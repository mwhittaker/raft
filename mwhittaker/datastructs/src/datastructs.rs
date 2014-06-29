struct Unit;
struct Tuple(int, int);
struct Struct{x: int, y: int}

mod boxes {
    pub struct WhiteBox<T> {
        pub contents: T
    }

    pub struct BlackBox<T> {
        contents: T
    }

    impl<T> BlackBox<T> {
        pub fn new(contents: T) -> BlackBox<T> {
            BlackBox { contents: contents }
        }
    }
}

fn main() {
    // Tuples
    
    // tuples can only be 12 items long
    let t = (1i, 2i, 3i, 4i, 5i, 6i, 7i, 8i, 9i, 10i, 11i, 12i);
    // let t = (1i, 2i, 3i, 4i, 5i, 6i, 7i, 8i, 9i, 10i, 11i, 12i, 13i);
    println!("{}", t);

    println!("{}", sort((1i, 2i)));
    println!("{}", sort((2i, 1i)));

    // Structs
    let _u = Unit;
    let _t = Tuple(1, 2);
    let _s = Struct{x: 3, y: 4};

    let Tuple(a, b) = _t;
    let Struct{x: c, y: d} = _s;

    println!("T({}, {}) and S({}, {})", a, b, c, d);

    struct Point{x: int, y: int};
    let mut p = Point{x: 1, y: 2};
    let pp = p;
    p.x = 22;
    println!("{}, {}", p.x, p.y);
    println!("{}, {}", pp.x, pp.y);

    // Struct visibility
    let _wb = boxes::WhiteBox{contents: 1i};
    let _bb = boxes::BlackBox::new(1i);

    // Boxes
    let mut boxed = box 2i;
    let box x = boxed;
    println!("{}", *boxed);
    println!("{}", x);
    *boxed += 1;
    println!("{}", *boxed);
}

// tuples, pattern matching, and guards
fn sort(t: (int, int)) -> (int, int) {
    match t {
        (x, y) if x <= y => (x, y),
        (x, y)           => (y, x)
    }
}
