//! Checks a variety of types to see if they are copyable

/// eat gobbles a value. If the value is non-copyable then you can only eat it
/// once
fn eat<T>(_: T) {}


struct Unit;
struct TupleStruct(int, int);
struct Struct{_x: int, _y: int}

fn main() {
    // copyable types
    let i = 1i;
    let t = (1i, 2i);
    let u = Unit;
    let ts = TupleStruct(1, 2);
    let s = Struct{_x: 1, _y: 2};
    let o = Some(1i);
    let r = &1i;

    eat(i); eat(i);
    eat(t); eat(t);
    eat(u); eat(u);
    eat(ts); eat(ts);
    eat(s); eat(s);
    eat(o); eat(o);
    eat(r); eat(r);

    // non-copyable types
    let _b = box 1i;
    let _bo = box Some(1i);
    
    // eat(_b); eat(_b);
    // eat(_bo); eat(_bo);
}
