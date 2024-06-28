use cli_table::{print_stdout, WithTitle};
use uuid::Uuid;

use opendut_carl_api::carl::CarlClient;
use opendut_types::topology::{DeviceDescription, DeviceId};

use crate::commands::device::DeviceTable;
use crate::DescribeOutputFormat;

/// Describe a device
#[derive(clap::Parser)]
pub struct DescribeDeviceCli {
    ///DeviceID
    #[arg()]
    id: Uuid,
}

impl DescribeDeviceCli {
    pub async fn execute(self, carl: &mut CarlClient, output: DescribeOutputFormat) -> crate::Result<()> {
        let device_id = DeviceId::from(self.id);

        let devices = carl.peers.list_devices().await
            .map_err(|_| String::from("Failed to fetch list of devices."))?;

        let device = devices.into_iter().find(|device| device.id == device_id)
            .ok_or(format!("Failed to find device for id <{}>", device_id))?;

        match output {
            DescribeOutputFormat::Table => {
                let table = [DeviceTable {
                    name: device.name,
                    id: device.id,
                    description: device.description.map(DeviceDescription::from).unwrap_or_default(),
                    tags: device.tags.iter().map(|tag| tag.value()).collect::<Vec<_>>().join(","),
                }];

                print_stdout(table.iter().with_title())
                    .expect("Newly created cluster configuration should be printable as table.");

            }
            DescribeOutputFormat::Json => {
                let json = serde_json::to_string(&device).unwrap();
                println!("{}", json);
                
            }
            DescribeOutputFormat::PrettyJson => {
                let json = serde_json::to_string_pretty(&device).unwrap();
                println!("{}", json);
            }
        };
        
        Ok(())
    }
}
