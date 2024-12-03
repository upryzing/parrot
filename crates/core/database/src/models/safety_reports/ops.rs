use upryzing_result::Result;

use crate::Report;

mod mongodb;
mod reference;

#[async_trait]
pub trait AbstractReport: Sync + Send {
    /// Insert a new report into the database
    async fn insert_report(&self, report: &Report) -> Result<()>;
}
