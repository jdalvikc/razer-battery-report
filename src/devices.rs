pub struct DeviceInfo {
    pub name: &'static str,
    pub pid: u16,
    pub interface: u8,
    pub usage_page: u16,
    pub usage: u16,
    pub vid: u16,
}

impl DeviceInfo {
    pub const fn new(
        name: &'static str,
        pid: u16,
        interface: u8,
        usage_page: u16,
        usage: u16,
    ) -> Self {
        DeviceInfo {
            name,
            pid,
            interface,
            usage_page,
            usage,
            vid: 0x1532,
        }
    }

    pub const fn transaction_id(&self) -> u8 {
        match self.pid {
            pid if pid == RAZER_DEATHADDER_V3_PRO_WIRED.pid
                || pid == RAZER_DEATHADDER_V3_PRO_WIRELESS.pid
                || pid == RAZER_DEATHADDER_V3_HYPERSPEED_WIRED.pid
                || pid == RAZER_DEATHADDER_V3_HYPERSPEED_WIRELESS.pid
                || pid == RAZER_BASILISK_V3_PRO_WIRED.pid
                || pid == RAZER_BASILISK_V3_PRO_WIRELESS.pid
                || pid == RAZER_BASILISK_V3_PRO_35K_WIRED.pid
                || pid == RAZER_BASILISK_V3_PRO_35K_WIRELESS.pid =>
            {
                0x1F
            }
            _ => 0x3F,
        }
    }
}

// Constantes existentes (mantén las que ya están en tu archivo)
pub const RAZER_DEATHADDER_V3_PRO_WIRED: DeviceInfo =
    DeviceInfo::new("Razer DeathAdder V3 Pro (Wired)", 0x00B6, 0, 1, 2);
pub const RAZER_DEATHADDER_V3_PRO_WIRELESS: DeviceInfo =
    DeviceInfo::new("Razer DeathAdder V3 Pro (Wireless)", 0x00B7, 0, 1, 2);

pub const RAZER_DEATHADDER_V3_HYPERSPEED_WIRED: DeviceInfo =
    DeviceInfo::new("Razer DeathAdder V3 HyperSpeed (Wired)", 0x00C4, 0, 1, 2);
pub const RAZER_DEATHADDER_V3_HYPERSPEED_WIRELESS: DeviceInfo =
    DeviceInfo::new("Razer DeathAdder V3 HyperSpeed (Wireless)", 0x00C5, 0, 1, 2);

pub const RAZER_DEATHADDER_V2_PRO_WIRED: DeviceInfo =
    DeviceInfo::new("Razer DeathAdder V2 Pro (Wired)", 0x007C, 0, 1, 2);
pub const RAZER_DEATHADDER_V2_PRO_WIRELESS: DeviceInfo =
    DeviceInfo::new("Razer DeathAdder V2 Pro (Wireless)", 0x007D, 0, 1, 2);

pub const RAZER_BASILISK_V3_PRO_WIRED: DeviceInfo =
    DeviceInfo::new("Razer Basilisk V3 Pro (Wired)", 0x00AA, 0, 1, 2);
pub const RAZER_BASILISK_V3_PRO_WIRELESS: DeviceInfo =
    DeviceInfo::new("Razer Basilisk V3 Pro (Wireless)", 0x00AB, 0, 1, 2);

pub const RAZER_VIPER_ULTIMATE_WIRED: DeviceInfo =
    DeviceInfo::new("Razer Viper Ultimate (Wired)", 0x007A, 0, 1, 2);
pub const RAZER_VIPER_ULTIMATE_WIRELESS: DeviceInfo =
    DeviceInfo::new("Razer Viper Ultimate (Wireless)", 0x007B, 0, 1, 2);

// Tus nuevas constantes para el 35K (Phantom Edition incluida, ya que usa los mismos PIDs)
pub const RAZER_BASILISK_V3_PRO_35K_WIRED: DeviceInfo =
    DeviceInfo::new("Razer Basilisk V3 Pro 35K (Wired)", 0x00D6, 0, 1, 2);

pub const RAZER_BASILISK_V3_PRO_35K_WIRELESS: DeviceInfo =
    DeviceInfo::new("Razer Basilisk V3 Pro 35K (Wireless)", 0x00D7, 0, 1, 2);

// Lista actualizada con tus dispositivos agregados al final
pub const RAZER_DEVICE_LIST: [DeviceInfo; 12] = [
    RAZER_DEATHADDER_V3_PRO_WIRED,
    RAZER_DEATHADDER_V3_PRO_WIRELESS,
    RAZER_DEATHADDER_V3_HYPERSPEED_WIRED,
    RAZER_DEATHADDER_V3_HYPERSPEED_WIRELESS,
    RAZER_DEATHADDER_V2_PRO_WIRED,
    RAZER_DEATHADDER_V2_PRO_WIRELESS,
    RAZER_BASILISK_V3_PRO_WIRED,
    RAZER_BASILISK_V3_PRO_WIRELESS,
    RAZER_VIPER_ULTIMATE_WIRED,
    RAZER_VIPER_ULTIMATE_WIRELESS,
    RAZER_BASILISK_V3_PRO_35K_WIRED,
    RAZER_BASILISK_V3_PRO_35K_WIRELESS,
];
