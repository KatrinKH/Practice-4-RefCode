use std::time::Duration;
use tokio::time::sleep;

use crate::app_state::AppState;

pub fn start(_state: AppState) {
    tokio::spawn(async move {
        loop {
            // Здесь потом будет логика ISS
            println!("Scheduler tick");

            sleep(Duration::from_secs(10)).await;
        }
    });
}
