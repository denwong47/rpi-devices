//! Async functions that are used to define when to terminate the program, to be used
//! with [`tokio::select!`].
//!

use std::time::Duration;

use rpi_errors::{IntoRPiResult, RPiResult};

/// Wait for a a `SIGTERM` to be issued.
pub async fn ctrl_c<'e>() -> RPiResult<'e, ()> {
    tokio::signal::ctrl_c().await.into_rpi_result()
}

/// Wait for a certain amount of time to pass; then returns [`Ok(())`].
pub async fn timeout<'e>(duration: Duration) -> RPiResult<'e, ()> {
    tokio::time::sleep(duration).await;

    Ok(())
}

/// Wait for a certain amount of time to pass; then returns [`Ok(())`]. If the
/// duration is [`None`], then wait forever.
pub async fn timeout_opt<'e>(duration_opt: Option<Duration>) -> RPiResult<'e, ()> {
    if let Some(duration) = duration_opt {
        tokio::time::sleep(duration).await;
    } else {
        // TODO find a better way to do this
        loop {
            tokio::time::sleep(Duration::MAX).await;
        }
    }

    Ok(())
}
