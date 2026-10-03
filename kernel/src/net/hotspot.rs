#[derive(Clone, Copy)]
pub struct NetworkFeatureController {
    pub wifi_available: bool,
    pub bluetooth_available: bool,
    pub ethernet_available: bool,
    pub hotspot_available: bool,
}

impl NetworkFeatureController {
    pub const fn new() -> Self {
        Self {
            wifi_available: false,
            bluetooth_available: false,
            ethernet_available: false,
            hotspot_available: false,
        }
    }
}
