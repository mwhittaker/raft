#![feature(macro_rules)] 

macro_rules! map(
    ()                                    => ({});
    ($a:expr)                             => ($a);
    ($a:expr -> $b:expr)                  => ($a.map($b));
    ($a:expr -> $b:expr -> $($c:expr)->*) => (map!($a.map($b) -> $($c)->*));
)

macro_rules! flatbind(
    ()                                    => ({});
    ($a:expr)                             => ($a);
    ($a:expr -> $b:expr)                  => ($a.and_then($b));
    ($a:expr -> $b:expr -> $($c:expr)->*) => (flatbind!($a.and_then($b) -> $($c)->*));
)

macro_rules! bind(
    () => ({});
    ($a:expr) => ($a);
    ($a:expr -> |$var:ident| $body:expr) => ($a.and_then(|$var| $body));
    ($a:expr -> |$var:ident| $body:expr -> $(|$vars:ident| $bodies:expr)->*) => ($a.and_then(|$var| {bind!($body -> $(|$vars| $bodies)->*)}));
)

fn main() {
    // Equivalent rust code:
    // Some("12345")
    // .map(|s| s.to_string())
    // .map(|s| s.len())
    // .map(|l| l * l)
    let o = map! {
        Some("12345")     ->
        |s| s.to_string() ->
        |s| s.len()       ->
        |l| l * l
    };
    assert!(o == Some(25));

    // Equivalent rust code:
    // Some("12345")
    // .and_then(|s| Some(s.to_string()))
    // .and_then(|s| Some(s.len()))
    // .and_then(|l| Some(l * l))
    let o = flatbind! {
        Some("12345")           ->
        |s| Some(s.to_string()) ->
        |s| Some(s.len())       ->
        |l| Some(l * l)
    };
    assert!(o == Some(25));

    // Equivalent OCaml code:
    // Some 3 >>= fun x ->
    // Some 4 >>= fun y ->
    // Some 5 >>= fun z ->
    // Some(z*z - x*x - y*y)
    //
    // Equivalent rust code:
    // Some(3i).and_then( |x| {
    //     Some(4i).and_then |y| {
    //         Some(5i).and_then |z| {
    //             Some(z*z - x*x - y*y)
    //         }
    //     }
    // })
    let o = bind! {
        Some(3i) -> |x| 
        Some(4i) -> |y| 
        Some(5i) -> |z| {
            assert!(x == 3i);
            assert!(y == 4i);
            assert!(z == 5i);
            Some(z*z - x*x - y*y)
        }
    };
    assert!(o == Some(0));
}
