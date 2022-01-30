use std::path::PathBuf;

use anyhow::Result;

pub struct DataCore {}

impl DataCore {
    fn create(path: &PathBuf) -> Result<Self> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn create__given_path__sqlite_db_created_at_path() {
        todo!()
    }
}
