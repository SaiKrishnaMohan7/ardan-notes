use tokio::runtime;

async fn hello() -> u32 {
    println!("Hello Tokio");
    6
}

async fn hello2() -> u32 {
    println!("Hello Tokio AGAIN");
    7
}

async fn ticker() {
    for i in 0..10 {
        println!("TICK: {i}");
    }
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
    // hello().await;

    // let (hello_1, hello_2) = tokio::join!(hello(), hello2());
    // println!("{hello_1}, {hello_2}");

    // tokio::spawn(ticker());
    // hello().await;

    let _ = tokio::join!(
        tokio::spawn(ticker()),
        tokio::spawn(hello()),
        tokio::spawn(ticker())
    );
}

// asynch rus cooperatively! So the results are not interleaved (only true for single threaded flavor = current_thread)
