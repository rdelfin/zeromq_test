pub mod data {
    tonic::include_proto!("zeromq_test.data");
}

pub mod capnp_structs {
    pub mod data {
        include!(concat!(env!("OUT_DIR"), "/capnp/data_capnp.rs"));
    }
}
