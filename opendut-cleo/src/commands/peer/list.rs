use std::fmt::{Display, Formatter};

use cli_table::{print_stdout, Table, WithTitle};
use serde::Serialize;

use opendut_carl_api::carl::CarlClient;
use opendut_types::peer::{PeerDescriptor, PeerId, PeerLocation, PeerName};
use crate::commands::peer::{PeerStatus, PeerTable};

use crate::ListOutputFormat;

/// List all peers
#[derive(clap::Parser)]
pub struct ListPeersCli;

impl ListPeersCli {
    pub async fn execute(self, carl: &mut CarlClient, output: ListOutputFormat) -> crate::Result<()> {
        let connected_peers = carl
            .broker
            .list_peers()
            .await
            .map_err(|error| format!("Could not list connected peers. {}", error))?;
        let all_peers = carl
            .peers
            .list_peer_descriptors()
            .await
            .map_err(|error| format!("Could not list peers.\n  {}", error))?;
        let peers_table = filter_connected_peers(&all_peers, &connected_peers);

        match output {
            ListOutputFormat::Table => {
                print_stdout(peers_table.with_title())
                    .expect("List of clusters should be printable as table.");
            }
            ListOutputFormat::Json => {
                let json = serde_json::to_string(&peers_table).unwrap();
                println!("{}", json);
            }
            ListOutputFormat::PrettyJson => {
                let json = serde_json::to_string_pretty(&peers_table).unwrap();
                println!("{}", json);
            }
        }
        Ok(())
    }
}

fn filter_connected_peers(
    all_peers: &[PeerDescriptor],
    connected_peers: &[PeerId],
) -> Vec<PeerTable> {
    all_peers
        .iter()
        .map(|peer| {
            let status = if connected_peers.contains(&peer.id) {
                PeerStatus::Connected
            } else {
                PeerStatus::Disconnected
            };
            let network_interfaces = Clone::clone(&peer.network.interfaces);
            let interfaces = network_interfaces.into_iter().map(|interface| interface.name.to_string()).collect::<Vec<_>>();
            PeerTable {
                name: Clone::clone(&peer.name),
                id: peer.id,
                location: Clone::clone(&peer.location.clone().unwrap_or_default()),
                network_interfaces: interfaces.join(", "),
                status
            }
        })
        .collect::<Vec<PeerTable>>()
}

#[cfg(test)]
mod test {
    use googletest::prelude::*;

    use opendut_types::peer::{PeerDescriptor, PeerId, PeerLocation, PeerName, PeerNetworkDescriptor};
    use opendut_types::peer::executor::ExecutorDescriptors;
    use opendut_types::util::net::{NetworkInterfaceConfiguration, NetworkInterfaceDescriptor, NetworkInterfaceName};

    use super::*;

    #[test]
    fn test() {
        let all_peers = vec![PeerDescriptor {
            id: PeerId::random(),
            name: PeerName::try_from("MyPeer").unwrap(),
            location: Some(PeerLocation::try_from("SiFi").unwrap()),
            network: PeerNetworkDescriptor{
                interfaces: vec!(NetworkInterfaceDescriptor {
                    name: NetworkInterfaceName::try_from("eth0").unwrap(),
                    configuration: NetworkInterfaceConfiguration::Ethernet,
                }),
                bridge_name: Some(NetworkInterfaceName::try_from("br-opendut-1").unwrap())
            },
            topology: Default::default(),
            executors: ExecutorDescriptors {
                executors: vec![]
            }
        }];
        let connected_peers = vec![all_peers[0].id];
        assert_that!(
            filter_connected_peers(&all_peers, &connected_peers),
            unordered_elements_are!(matches_pattern!(PeerTable {
                name: eq(Clone::clone(&all_peers[0].name)),
                id: eq(all_peers[0].id),
                status: eq(PeerStatus::Connected),
            }))
        );
    }
}
