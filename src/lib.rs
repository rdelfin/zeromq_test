// Include the `items` module, which is generated from items.proto.
pub mod data {
    include!(concat!(env!("OUT_DIR"), "/zeromq_test.data.rs"));
}
