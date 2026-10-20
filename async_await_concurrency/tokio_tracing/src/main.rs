use anyhow::Ok;
use tracing_subscriber::fmt::format::FmtSpan;
#[tracing::instrument]
async fn hello_world() {
    println!("hello, world!");
}

// tokio-console good for neat profiling
// OTEL integration
// ^ these are all diff types of available subscribers, including for other systems
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let subscriber = tracing_subscriber::FmtSubscriber::new(); // there are a lot of otehr options to do this in a custom way.
    // Thing: you can use launchd or systemd to setup log rotation on servers

    // Start configuring a `fmt` subscriber
    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT)
        // Build the subscriber
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    tracing::info!("Starting up gang");
    tracing::warn!("ZZZZ");
    tracing::error!("Horrid, dead");
    Ok(())
}

// https://github.com/tokio-rs/console
