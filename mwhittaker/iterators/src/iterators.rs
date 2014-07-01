// Even
struct Even {
    x: uint
}

impl Iterator<uint> for Even {
    fn next(&mut self) -> Option<uint> {
        self.x += 2;
        Some(self.x - 2)
    }
}

impl Even {
    fn new() -> Even {
        Even{x: 0}
    }
}

// List
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>)
}

struct ListIterator<'a, T> {
    l: &'a List<T>
}

impl<T> List<T> {
    fn iter<'a>(&'a self) -> ListIterator<'a, T> {
        ListIterator{l: self}
    }
}

impl<'a, T> Iterator<&'a T> for ListIterator<'a, T> {
    fn next(&mut self) -> Option<&'a T> {
        match *self.l {
            Nil => None,
            Cons(ref x, box ref xs) => {
                self.l = xs;
                Some(x)
            }
        }
    }
}

fn main() {
    for i in Even::new().take(10) {
        println!("{}", i);
    }

    let l: List<int> = Cons(2, box Cons(1, box Nil));
    for x in l.iter() {
        println!("{}", x);
    }
}
