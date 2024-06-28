use cli_table::Table;
use opendut_types::cluster::{ClusterId, ClusterName};
use opendut_types::peer::PeerId;

pub mod create;
pub mod list;
pub mod describe;
pub mod delete;

#[derive(Table)]
pub struct ClusterConfigTable {
    #[table(title = "Name")]
    name: ClusterName,
    #[table(title = "ClusterID")]
    id: ClusterId,
    #[table(title = "Leader")]
    leader: PeerId,
    #[table(title = "Devices")]
    devices: String,
}
