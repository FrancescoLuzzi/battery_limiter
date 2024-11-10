use async_fs::OpenOptions;
use futures_lite::AsyncWriteExt;
use std::{
    fs::OpenOptions as SyncOpenOptions,
    io::{self, Read as _},
};

use crate::THRESHOLD_FILE;

#[derive(Debug, Clone, Copy)]
pub enum BatteryLevel {
    Low,
    Medium,
    Full,
    Custom(u8),
}

impl From<u8> for BatteryLevel {
    fn from(value: u8) -> Self {
        match value {
            60 => Self::Low,
            80 => Self::Medium,
            100 => Self::Full,
            num if num < 100 => Self::Custom(num),
            // I could panic or implement TryFrom, at the moment idk
            _ => Self::Full,
        }
    }
}

impl BatteryLevel {
    pub fn from_system() -> io::Result<BatteryLevel> {
        let mut end_threshold_file = SyncOpenOptions::new().read(true).open(THRESHOLD_FILE)?;
        let mut buffer = String::with_capacity(3);
        end_threshold_file.read_to_string(&mut buffer)?;
        let percentage: u8 = buffer.trim().parse().expect("percentage found is not a u8");
        Ok(percentage.into())
    }

    pub fn get_percentage(&self) -> u8 {
        match self {
            BatteryLevel::Low => 60,
            BatteryLevel::Medium => 80,
            BatteryLevel::Full => 100,
            BatteryLevel::Custom(l) => (*l).clamp(0, 100),
        }
    }

    pub fn get_gtk_icon_name(&self) -> &'static str {
        match self.get_percentage() {
            0..=10 => "battery-level-0",
            11..=20 => "battery-level-10",
            21..=30 => "battery-level-20",
            31..=40 => "battery-level-30",
            41..=50 => "battery-level-40",
            51..=60 => "battery-level-50",
            61..=70 => "battery-level-60",
            71..=80 => "battery-level-70",
            81..=90 => "battery-level-80",
            91..100 => "battery-level-90",
            100 => "battery-level-100",
            // I could return an Option or create a custom type instead of u8, at the moment idk
            default => panic!("percentage {default} not supported"),
        }
    }

    pub async fn apply(&self) -> io::Result<u8> {
        let mut end_threshold_file = OpenOptions::new().append(true).open(THRESHOLD_FILE).await?;
        let percentage = self.get_percentage();
        end_threshold_file
            .write_all(percentage.to_string().as_bytes())
            .await?;
        end_threshold_file.sync_data().await?;
        Ok(percentage)
    }
}
