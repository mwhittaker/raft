extern crate capnp;
use std::io::{stdin, stdout, IoResult};
use capnp::{MessageBuilder, MessageReader, ReaderOptions, MallocMessageBuilder};
use capnp::serialize_packed;
use polygon_capnp::{Polygon};

mod polygon_capnp;

fn write() {
    let mut message = MallocMessageBuilder::new_default(); 
    
    {
        let polygon = message.init_root::<Polygon::Builder>();
        let points = polygon.init_points(3);

        let a = points.get(0);
        a.set_x(0);
        a.set_y(0);

        let b = points.get(1);
        b.set_x(1);
        b.set_y(1);

        let c = points.get(2);
        c.set_x(1);
        c.set_y(0);
    }

    let _ = serialize_packed::write_packed_message_unbuffered(&mut stdout(), &message);
}

fn read() -> IoResult<()> {
    let message_reader = try!(serialize_packed::new_reader_unbuffered(&mut stdin(), ReaderOptions::new()));
    let polygon = message_reader.get_root::<Polygon::Reader>();
   
    let points = polygon.get_points();
    for i in range(0, points.size()) {
        let point = points.get(i);
        println!("({}, {})", point.get_x(), point.get_y());
    }

    Ok(())
}

fn main() {
    let usage = || { println!("usage: polygon [read|write]"); };

    match std::os::args().as_slice() {
        [] => fail!("impossible"),
        [_] => usage(),
        [_, ref cmd, ..] => {
            match cmd.as_slice() {
                "read" => {let _ = read();},
                "write" => write(),
                _ => usage()
            }
        }    
    } 
}
