use argh::FromArgs;

#[derive(FromArgs)]
/// change battery charge threshold
pub struct BatteryLimiterArgs {
    /// persist change as systemd service
    #[argh(switch)]
    pub persist: bool,

    /// battery percentage threshold
    #[argh(option, short = 'p')]
    pub percentage: u8,
}
