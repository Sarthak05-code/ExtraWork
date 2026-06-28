use std::time::Duration;
use tokio::sync::{mpsc};
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

// Custom Error Type for robust error handling
#[derive(Debug)]
enum AppError {
    FetchFailed,
}

#[tokio::main]
async fn main() {
    // 1. Initialize structured tracing/logging
    tracing_subscriber::fmt::init();
    tracing::info!("Starting the elevated async application...");

    // 2. Setup a CancellationToken for Graceful Shutdown
    let cancel_token = CancellationToken::new();
    
    // 3. Setup an MPSC channel for sending metrics/pings from background to main
    let (tx, mut rx) = mpsc::channel::<u32>(100);

    // Spawn the background runner, passing the cancellation token and channel transmitter
    let background_cloned_token = cancel_token.clone();
    tokio::spawn(async move {
        background_runner(background_cloned_token, tx).await;
    });

    // --- Main Logic Execution ---
    sleep(Duration::from_secs(3)).await;
    tracing::info!("Fetching initial data...");

    match fetch_data().await {
        Ok(data) => tracing::info!(target: "network_events", "Received: {}", data),
        Err(e) => tracing::error!("Failed to fetch data: {:?}", e),
    }

    // Execute concurrent tasks
    do_stuffs().await;

    // 4. Graceful Shutdown Trigger
    tracing::info!("Initiating graceful shutdown of background tasks...");
    cancel_token.cancel(); // Signals the background loop to break

    // Receive the final metrics report from the background task before exiting
    let final_seconds = rx.recv().await.unwrap_or(0);
    
    tracing::info!("Program finished cleanly.");
    tracing::info!("Background task ran for total of {} seconds.", final_seconds);
}

/// Background runner that respects cancellation and reports back via a channel
async fn background_runner(token: CancellationToken, tx: mpsc::Sender<u32>) {
    let mut seconds_elapsed = 0;

    loop {
        tokio::select! {
            // Check if shutdown was requested
            _ = token.cancelled() => {
                tracing::info!("Background runner received shutdown signal. Sending final metrics...");
                let _ = tx.send(seconds_elapsed).await; // Send final count back to main
                break;
            }
            // Otherwise, perform the periodic tick
            _ = sleep(Duration::from_secs(1)) => {
                seconds_elapsed += 1;
                tracing::debug!("Background tick: {}s", seconds_elapsed);
            }
        }
    }
}

/// Executes concurrent operations handling idiomatic Results
async fn do_stuffs() {
    tracing::info!("Starting concurrent data fetches...");
    
    let task_one = fetch_data();
    let task_two = fetch_data();

    // Concurrently await both tasks
    let (res1, res2) = tokio::join!(task_one, task_two);

    match (res1, res2) {
        (Ok(r1), Ok(r2)) => {
            tracing::info!("Both concurrent tasks succeeded!");
            tracing::info!("Task 1: {}, Task 2: {}", r1, r2);
        }
        _ => tracing::error!("One or more concurrent tasks failed."),
    }
}

/// Simulates a real-world network fetch with robust Result types
async fn fetch_data() -> Result<String, AppError> {
    sleep(Duration::from_secs(2)).await;
    
    // Simulate a successful fetch 90% of the time
    if rand::random::<f32>() > 0.1 {
        Ok(String::from("Hello from the elevated async world!"))
    } else {
        Err(AppError::FetchFailed)
    }
}