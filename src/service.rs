use crate::{SERVICE_FILE, THRESHOLD_FILE};
use askama::Template;
use async_fs::OpenOptions;
use async_process::{Command, ExitStatus};
use futures_lite::AsyncWriteExt;

#[derive(Template)]
#[template(path = "battery_limiter.service")]
pub struct BatteryLimiterService<'a> {
    // TODO: consider adding custom cli command and bypass the shell entirely
    events: &'a [&'a str],
    battery_threshold_path: &'a str,
    battery_threshold: u8,
}

impl BatteryLimiterService<'_> {
    pub fn new(battery_threshold: u8) -> Self {
        Self {
            battery_threshold,
            ..Default::default()
        }
    }

    pub async fn persist(&self) -> std::io::Result<ExitStatus> {
        save_service(self, SERVICE_FILE).await?;
        //TODO: handle the status results
        Command::new("/usr/bin/systemctl")
            .arg("daemon-reload")
            .status()
            .await?;
        Command::new("/usr/bin/systemctl")
            .args(["enable", "battery_limiter.service"])
            .status()
            .await
    }
}

impl Default for BatteryLimiterService<'_> {
    fn default() -> Self {
        Self {
            events: &[
                "hibernate.target",
                "hybrid-sleep.target",
                "multi-user.target",
                "suspend.target",
                "suspend-then-hibernate.target",
            ],
            battery_threshold_path: THRESHOLD_FILE,
            battery_threshold: Default::default(),
        }
    }
}

async fn save_service<'a>(
    service: &BatteryLimiterService<'a>,
    dest_path: &str,
) -> Result<(), std::io::Error> {
    let mut file_out = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(dest_path)
        .await?;
    file_out
        .write_all(
            service
                .render()
                .expect("something went wrong rendering battery service")
                .as_bytes(),
        )
        .await?;
    file_out.sync_data().await
}
