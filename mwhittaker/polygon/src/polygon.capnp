@0xb199b129df9db906;

struct Point {
    x @0 :Int32;
    y @1 :Int32;
}

struct Polygon {
    points @0 :List(Point);
}
