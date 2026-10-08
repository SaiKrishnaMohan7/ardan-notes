use tokio::runtime;

async fn hello() {
    println!("Hello Tokio");
}

// Long way of setting up tokio
// fn main() {
//     // single threaded
//     let rt = runtime::Builder::new_current_thread()
//         .enable_all()
//         .build()
//         .unwrap();

//     let rt = runtime::Builder::new_multi_thread()
//         .enable_all()
//         .worker_threads(4) // num cores we want to dedicate to task; one worker thread per core default
//         .build()
//         .unwrap();

//     rt.block_on(hello());
// }

// short way of setting up tokio
// default is multithreaded, highest performance setting
// Some level of modification can be done with #[tokio::main(flavor = "current_thread")] for single thread, for example
// For minute customization a mix and match approach OR the long form approach is the best
#[tokio::main]
async fn main() {
    hello().await;
}
