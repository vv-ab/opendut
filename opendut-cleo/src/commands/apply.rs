use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

use opendut_carl_api::carl::CarlClient;
use opendut_types::specs::{Specification, SpecificationDocument, yaml};
use yaml::YamlSpecificationDocument;

use crate::CreateOutputFormat;

#[derive(clap::Parser)]
///Create openDuT resource form file
pub struct ApplyCli {
    #[arg(short, long, value_parser=parse_source)]
    from: Source,
    ///Text, JSON or prettified JSON as output format
    #[arg(global=true, value_enum, short, long, default_value_t=CreateOutputFormat::Text)]
    output: CreateOutputFormat,
}

impl ApplyCli {
    pub async fn execute(self, carl: &mut CarlClient) -> crate::Result<()> {
        match self.from {
            Source::File(path) => {
                let content = fs::read_to_string(path).unwrap();
                let documents = serde_yaml::Deserializer::from_str(&content)
                    .map(|yaml_document| {
                        serde_yaml::Value::deserialize(yaml_document)
                            .and_then(|value| serde_yaml::from_value::<YamlSpecificationDocument>(value))
                    })
                    .collect::<Result<Vec<_>, _>>();

                match documents {
                    Ok(values) => {
                        values.into_iter().for_each(|yaml_specification| {
                            let spec = SpecificationDocument::try_from(yaml_specification).unwrap();
                            // match spec {
                            //     Specification::PeerDescriptorSpecification(PeerDescriptorSpecification::V1(peer)) => {
                            // 
                            // 
                            // 
                            //     }
                            //     Specification::ClusterConfigurationSpecification(_) => {}
                            // }
                            println!("{spec:?}");
                        });
                        Ok(())
                    }
                    Err(cause) => {
                        Err(format!("Failed to parse specification: {cause}"))
                    }
                }
            }
            Source::Inline(InlineSource::Json(json)) => {
                match Specification::from_json_str(json.as_str()) {
                    Ok(_) => {}
                    Err(_) => {}
                }
                todo!("unsupported source")
            }
            Source::Url(_) => {
                todo!("unsupported source")
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Source {
    File(PathBuf),
    Url(String),
    Inline(InlineSource)
}

#[derive(Clone, Debug)]
pub enum InlineSource {
    Json(String)
}

#[derive(thiserror::Error, Debug)]
#[error("'{arg}' cannot be used as source. {details}")]
struct SourceParsingError {
    arg: String,
    details: String,
}

fn parse_source(arg: &str) -> Result<Source, SourceParsingError> {

    if arg.starts_with('{') && arg.ends_with('}') {
        Ok(Source::Inline(InlineSource::Json(arg.to_owned())))
    }
    else {
        let path = PathBuf::from(arg);
        if !path.exists() {
            Err(SourceParsingError { arg: String::from(arg), details: String::from("Path does not exist.") })?;
        }
        if path.is_file() || path.is_dir() {
            Ok(Source::File(path))
        }
        else {
            Err(SourceParsingError { arg: String::from(arg), details: String::from("Path designates neither a file nor a directory.") })
        }
    }
}