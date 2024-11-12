pub mod args;
pub mod battery_level;
pub mod service;

const SERVICE_FILE: &str = "/etc/systemd/system/battery_limiter.service";
const THRESHOLD_FILE: &str = "/sys/class/power_supply/BAT0/charge_control_end_threshold";
