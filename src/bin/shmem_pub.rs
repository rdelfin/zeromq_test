use raw_sync::{events::*, Timeout};
use shared_memory::*;

fn main() -> anyhow::Result<()> {
    // Attempt to create a mapping or open if it already exists
    println!("Getting the shared memory mapping");
    let shmem = ShmemConf::new()
        .size(4096)
        .os_id("event_mapping")
        .create()?;

    //Create an event in the shared memory
    println!("Creating event in shared memory");
    let (evt, used_bytes) = unsafe { Event::new(shmem.as_ptr(), true) }.unwrap();
    println!("\tUsed {} bytes", used_bytes);

    println!("Launch another instance of this example to signal the event !");
    evt.wait(Timeout::Infinite).unwrap();
    println!("\tGot signal !");

    println!("Done !");
    Ok(())
}
