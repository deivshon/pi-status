use crate::status::DOCKER_NET_DIR_ENV;

use lazy_static::lazy_static;

pub const NET_DIR_DEFAULT: &str = "/sys/class/net/";

pub const RX_DIR: &str = "statistics/tx_bytes";
pub const TX_DIR: &str = "statistics/rx_bytes";

lazy_static! {
    pub static ref NET_DIR: String = if let Ok(net_dir) = std::env::var(DOCKER_NET_DIR_ENV) {
        net_dir
    } else {
        String::from(NET_DIR_DEFAULT)
    };
}
