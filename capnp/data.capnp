@0xcf5f08213292f34e;

struct Action {
    timestamp @0 :UInt64;
    angle @1 :Float64;
    speed @2 :Float64;
}

struct Image {
    timestamp @0 :UInt64;
    width @1 :UInt32;
    height @2 :UInt32;
    channels @3 :UInt32;
    data @4 :Data;
}
