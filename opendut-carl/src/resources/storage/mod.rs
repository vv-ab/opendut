use url::Url;
use crate::resources::{IntoId, Iter, IterMut, Resource, Update};
use crate::resources::storage::database::ResourcesDatabaseStorage;
use crate::resources::storage::memory::ResourcesMemoryStorage;

pub(super) mod memory;
pub(super) mod database;

pub enum ResourcesStorage {
    Database(ResourcesDatabaseStorage),
    Memory(ResourcesMemoryStorage),
}
impl ResourcesStorage {
    pub fn connect(options: ResourcesStorageOptions) -> Result<Self, ConnectionError> {
        let storage = match options {
            ResourcesStorageOptions::Database { url } => {
                let storage = ResourcesDatabaseStorage::connect(&url)
                    .map_err(|cause| ConnectionError::Database { url, cause })?;
                ResourcesStorage::Database(storage)
            }
            ResourcesStorageOptions::Memory => {
                ResourcesStorage::Memory(ResourcesMemoryStorage::default())
            }
        };
        Ok(storage)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error("Failed to connect to database at '{url}':\n  {cause}")]
    Database { url: Url, cause: diesel::ConnectionError },
}

#[derive(Clone)]
pub enum ResourcesStorageOptions {
    Database {
        url: Url,
    },
    Memory,
}
impl ResourcesStorageOptions {
    pub fn load(config: &config::Config) -> Result<Self, opendut_util::settings::LoadError> {
        let persistence_enabled = config.get_bool("persistence.enabled")?;

        if persistence_enabled {
            let url_field = "persistence.database.url";
            let url_value = config.get_string(url_field)?;
            let url = Url::parse(&url_value)
                .map_err(|cause| opendut_util::settings::LoadError::Parse {
                    field: url_field.to_string(),
                    value: url_value,
                    source: Box::new(cause)
                })?;
            Ok(ResourcesStorageOptions::Database { url })
        } else {
            Ok(ResourcesStorageOptions::Memory)
        }
    }
}

pub trait ResourcesStorageApi {
    fn insert<R>(&mut self, id: impl IntoId<R>, resource: R) -> Option<R>
    where R: Resource;

    fn update<R>(&mut self, id: impl IntoId<R>) -> Update<R>
    where R: Resource;

    fn remove<R>(&mut self, id: impl IntoId<R>) -> Option<R>
    where R: Resource;

    fn get<R>(&self, id: impl IntoId<R>) -> Option<R>
    where R: Resource + Clone;

    fn iter<R>(&self) -> Iter<R>
    where R: Resource;

    fn iter_mut<R>(&mut self) -> IterMut<R>
    where R: Resource;
}
