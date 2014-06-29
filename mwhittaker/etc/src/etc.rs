type Point = (f32, f32);

fn main() {
    let one: int = 0b1;
    let ten: int = 0b10;
    println!("1 in binary is {0:t}. 0b10 is decimal is {1}", one, ten);

    let mut vec = Vec::new();
    vec.push(2i);
    vec.push(3i);
    println!("{}", vec);

    let mut vec2 = vec!("a", "b", "c");
    vec2.push("d");
    println!("{}", vec2);

    let p: Point = (1.0, 2.0);
    println!("{}", p);

    let x = if true { 1i } else { 2i };
    println!("x = {}", x);

    fizzbuzz(5);

    for i in range(0u, 10) {
        println!("{}", i);
    }
}

fn fizzbuzz(n: uint) {
    let mut i = 1u;

    while i < n {
        if i % 15 == 0 {
            println!("fizzbuzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }

        i += 1;
    }
}
