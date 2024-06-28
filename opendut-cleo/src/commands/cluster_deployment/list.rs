use cli_table::{print_stdout, WithTitle};

use opendut_carl_api::carl::CarlClient;

use crate::commands::cluster_deployment::ClusterDeploymentTable;
use crate::ListOutputFormat;

/// List all cluster deployments
#[derive(clap::Parser)]
pub struct ListClusterDeploymentsCli;

impl ListClusterDeploymentsCli {
    pub async fn execute(self, carl: &mut CarlClient, output: ListOutputFormat) -> crate::Result<()> {
        let clusters = carl.cluster.list_cluster_deployments().await
            .map_err(|error| format!("Error while listing cluster deployments: {}", error))?;

        match output {
            ListOutputFormat::Table => {
                let cluster_table = clusters.into_iter()
                    .map(|cluster_deployment| {
                        ClusterDeploymentTable {
                            id: cluster_deployment.id,
                        }
                    })
                    .collect::<Vec<_>>();
                print_stdout(cluster_table.with_title())
                    .expect("List of clusters should be printable as table.");
            }
            ListOutputFormat::Json => {
                let json = serde_json::to_string(&clusters).unwrap();
                println!("{}", json);
            }
            ListOutputFormat::PrettyJson => {
                let json = serde_json::to_string_pretty(&clusters).unwrap();
                println!("{}", json);
            }
        }

        Ok(())
    }
}
