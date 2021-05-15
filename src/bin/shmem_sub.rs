use raw_sync::{events::*, Timeout};
use shared_memory::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Attempt to create a mapping or open if it already exists
    println!("Getting the shared memory mapping");
    let shmem = ShmemConf::new().os_id("event_mapping").open()?;

    // Open existing event
    println!("Openning event from shared memory");
    let (evt, used_bytes) = unsafe { Event::from_existing(shmem.as_ptr())? };
    println!("\tEvent uses {} bytes", used_bytes);

    println!("Signaling event !");
    evt.set(EventState::Signaled)?;
    println!("\tSignaled !");

    println!("Done !");
    Ok(())
}
