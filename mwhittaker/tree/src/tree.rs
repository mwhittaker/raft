pub enum Tree<T> {
    Leaf,
    Node(Box<Tree<T>>, T, Box<Tree<T>>)
}

impl<T, U: Copy> Tree<T> {
    fn fold(self, f: |l: U, x: T, r: U| -> U, acc: U) -> U {
        match self {
            Leaf => acc,
            Node(box l, x, box r) => {
                let l = l.fold(|l,x,r| {f(l,x,r)}, acc);
                let r = r.fold(|l,x,r| {f(l,x,r)}, acc);
                f(l, x, r)
            }
        }
    }
}

fn main() {
    let tl = Node(box Leaf, 1i, box Leaf);
    let tr = Node(box Leaf, 2i, box Leaf);
    let t  = Node(box tl, 3i, box tr);

    println!("sum(t) == {}", t.fold(|l,x,r|{l + x + r}, 0))
}
