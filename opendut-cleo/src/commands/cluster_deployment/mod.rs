use cli_table::Table;
use opendut_types::cluster::{ClusterId};

pub mod create;
pub mod list;
pub mod delete;

#[derive(Table)]
pub struct ClusterDeploymentTable {
    #[table(title = "ClusterID")]
    id: ClusterId,
}