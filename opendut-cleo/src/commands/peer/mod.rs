use std::fmt::{Display, Formatter};
use cli_table::Table;
use serde::Serialize;
use opendut_types::peer::{PeerId, PeerLocation, PeerName};

pub mod list;
pub mod describe;
pub mod create;
pub mod delete;


#[derive(Table, Debug, Serialize)]
struct PeerTable {
    #[table(title = "Name")]
    name: PeerName,
    #[table(title = "PeerID")]
    id: PeerId,
    #[table(title = "Status")]
    status: PeerStatus,
    #[table(title = "Location")]
    location: PeerLocation,
    #[table(title = "NetworkInterfaces")]
    network_interfaces: String,
}


#[derive(Debug, PartialEq, Serialize)]
pub enum PeerStatus {
    Connected,
    Disconnected,
}

impl Display for PeerStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PeerStatus::Connected => write!(f, "Connected"),
            PeerStatus::Disconnected => write!(f, "Disconnected"),
        }
    }
}
