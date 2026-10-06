use std::{sync::LazyLock, thread, time::Duration};

use dashmap::DashMap;

static SHARED: LazyLock<DashMap<u32, u32>> = LazyLock::new(DashMap::new);

fn main() {
    for n in 0..100 {
        thread::spawn(move || {
            if let Some(mut v) = SHARED.get_mut(&n) {
                *v += 1;
            } else {
                SHARED.insert(n, n);
            }
        });
    }
    thread::sleep(Duration::from_secs(2));
    println!("{SHARED:#?}");
}
