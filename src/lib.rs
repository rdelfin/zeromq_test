pub mod data {
    include!(concat!(env!("OUT_DIR"), "/zeromq_test.data.rs"));
}

pub mod capnp_structs {
    pub mod data {
        include!(concat!(env!("OUT_DIR"), "/capnp/data_capnp.rs"));
    }
}

pub mod lcm_structs {
    include!(concat!(env!("OUT_DIR"), "/lcm_data/mod.rs"));
}
