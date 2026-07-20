use shared_frontend::i18n::Language;
use shared_frontend::locale::{detect_browser_locale, set_saved_locale};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum PulseKey {
    Cpu,
    Memory,
    Storage,
    Network,
    Gpu,
    SystemInfo,
    Uptime,
    Hostname,
    Os,
    Kernel,
    EnterPin,
    TooManyAttempts,
    AttemptsRemaining(usize),
    Ready,
    CpuLoadHigh(f32),
    CpuTempHigh(f32),
    RamSpaceLow(f32),
    DiskSpaceLow(f32),
    GpuLoadHigh(usize, f32),
    GpuTempHigh(usize, f32),
    HighNetworkTraffic(String),
    SystemRecentlyRebooted(String),
    Disconnected,
    ConnectionEstablished,
}
