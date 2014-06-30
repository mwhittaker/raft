//! A package that aims to clear up the differences between closures and procs. 
fn main() {
    // First up, closures. Closures borrow the values they capture. Here, f
    // will immutably borrow b. Thus, I can print b from within the closure and
    // from outside it.
    let b = box 1u;
    let f = || {println!("captured b = {}", b)};
    println!("non-captured b = {}", b);
    f();

    // Now, let's mutably borrow something and try to capture it in a closure
    let mut b = box 1u;
    let _r = &mut b;
    // let f = || {println!("captured b = {}", b)};
    // Try uncommenting this line above. It won't compile because f cannot
    // immutably borrow b since _r mutably borrows it.

    // Okay, now let's look at closures. Closures will move captured values
    // instead of borrowing them.
    let b = box 1u;
    let _f = proc() {println!("proc captured b = {}", b)};
    // println!("non-captured b = {}", b);
    // Try uncommenting the line above. We can't print b because b is
    // non-copyable and was moved into _f.

    // Just for fun
    for i in range(0u, 5) {
        spawn(proc() {println!("hello from proc {}", i)});
    }
}
