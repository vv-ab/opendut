use cli_table::{print_stdout, WithTitle};
use uuid::Uuid;

use opendut_carl_api::carl::CarlClient;
use opendut_types::cluster::ClusterId;
use crate::commands::cluster_configuration::ClusterConfigTable;

use crate::DescribeOutputFormat;

/// Describe a cluster configuration
#[derive(clap::Parser)]
pub struct DescribeClusterConfigurationCli {
    ///ClusterID
    #[arg()]
    id: Uuid,
}

impl DescribeClusterConfigurationCli {
    pub async fn execute(self, carl: &mut CarlClient, output: DescribeOutputFormat) -> crate::Result<()> {
        let cluster_id = ClusterId::from(self.id);

        let clusters_configuration = carl.cluster.list_cluster_configurations().await
            .map_err(|_| String::from("Failed to get list of cluster configurations!"))?;

        let cluster_configuration = clusters_configuration.into_iter()
            .find(|cluster_configuration| cluster_configuration.id == cluster_id)
            .ok_or(format!("Failed to find cluster configuration for ClusterID <{}>", cluster_id))?;

        let cluster_devices = {
            let devices = carl.peers.list_devices().await
                .map_err(|_| String::from("Failed to get list of devices!"))?;
            devices.into_iter()
                .filter(|device| cluster_configuration.devices.contains(&device.id))
                .map(|devices| devices.name)
                .collect::<Vec<_>>()
        };

        let cluster_peers = {
            let peers = carl.peers.list_peer_descriptors().await
                .map_err(|_| String::from("Failed to get list of peers!"))?;
            peers.into_iter()
                .filter(|peer| {
                    peer.topology.devices.iter().any(|device| cluster_devices.contains(&device.name))
                })
                .map(|peer| peer.name)
                .collect::<Vec<_>>()
        };

        match output {
            DescribeOutputFormat::Table => {
                let table = [ClusterConfigTable {
                    name: cluster_configuration.clone().name,
                    id: cluster_id,
                    leader: cluster_configuration.leader,
                    devices: cluster_devices.iter().map(|name| name.value()).collect::<Vec<_>>().join(","),
                }];
                print_stdout(table.iter().with_title())
                    .expect("Cluster configuration should be printable as table");
            }
            DescribeOutputFormat::Json => {
                let json = serde_json::to_string(&cluster_configuration).unwrap();
                println!("{}", json)
            }
            DescribeOutputFormat::PrettyJson => {
                let json = serde_json::to_string_pretty(&cluster_configuration).unwrap();
                println!("{}", json);
            }
        }
            

        Ok(())
    }
}
