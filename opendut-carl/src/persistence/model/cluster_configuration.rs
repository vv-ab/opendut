use opendut_types::cluster::ClusterConfiguration;
use opendut_types::resources::Id;
use crate::persistence::database::Db;
use super::{Persistable, PersistableConversionError};

#[derive(Debug)] //diesel::Queryable, diesel::Selectable, diesel::Insertable)]
// #[diesel(table_name = crate::persistence::database::schema::cluster_configuration)] //TODO create schema
// #[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PersistableClusterConfiguration {
    //TODO
}
impl Persistable<ClusterConfiguration> for PersistableClusterConfiguration {
    fn insert(&self, db: Db) -> Option<Self> {
        todo!()
    }

    fn get(id: &Id, db: Db) -> Option<Self> {
        todo!()
    }
}

impl From<ClusterConfiguration> for PersistableClusterConfiguration {
    fn from(value: ClusterConfiguration) -> Self {
        todo!()
    }
}
impl TryFrom<PersistableClusterConfiguration> for ClusterConfiguration {
    type Error = PersistableConversionError<PersistableClusterConfiguration, ClusterConfiguration>;

    fn try_from(value: PersistableClusterConfiguration) -> Result<Self, Self::Error> {
        todo!()
    }
}
