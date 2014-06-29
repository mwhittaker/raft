enum List {
    Nil,
    Cons(int, Box<List>)
}

impl List {
    fn len(&self) -> uint {
        match *self {
            Nil => 0,
            Cons(_, ref t) => 1 + t.len()
        }
    }

    fn append(self, x: int) -> List {
        match self {
            Nil => Cons(x, box Nil),
            l   => Cons(x, box l)
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Nil              => format!("Nil"),
            Cons(x, box Nil) => format!("{}", x),
            Cons(x, ref xs)  => format!("{}, {}", x, xs.stringify())
        }
    }
}

fn main() {
    let mut l = Nil;
    for i in range(0u, 10) {
        l = l.append(i as int);
    }
    println!("{}", l.len());
    println!("{}", l.stringify());
}

#[test]
fn test_len() {
    let l0 = Nil;
    let l1 = Cons(0, box Nil);
    let l2 = Cons(0, box Cons(0, box Nil));
    let l3 = Cons(0, box Cons(0, box Cons(0, box Nil)));

    assert!(l0.len() == 0);
    assert!(l1.len() == 1);
    assert!(l2.len() == 2);
    assert!(l3.len() == 3);
}

#[test]
fn test_append() {
    let mut l = Nil;
    for i in range(0u, 10) {
        l = l.append(i as int);
    }
    assert!(l.len() == 10);
}

#[test]
fn test_stringify() {
    let l0 = Nil;
    let l1 = Cons(0, box Nil);
    let l2 = Cons(0, box Cons(0, box Nil));

    assert!(l0.stringify() == format!("Nil"));
    assert!(l1.stringify() == format!("0"));
    assert!(l2.stringify() == format!("0, 0"));
}
