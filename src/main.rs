use std::thread;

fn main() {
    let core_ids = core_affinity::get_core_ids().unwrap();

    let handles = core_ids
        .into_iter()
        .map(|id| {
            thread::spawn(move || {
                let success = core_affinity::set_for_current(id);
                if success {
                    println!("Hello from core {id:?}");
                } else {
                    println!("Unable to set affinity to core {id:?}");
                }
            })
        })
        .collect::<Vec<_>>();

    for handle in handles.into_iter() {
        handle.join().unwrap();
    }
}

// Affinity: binding a thread to cores manually
// The schedulers have become really smart though so binding a thread to a core maybe slow.
// Run benchmark
