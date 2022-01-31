use std::path::Path;

use anyhow::Result;

pub struct DataCore {}

impl DataCore {
    pub fn create<P: AsRef<Path>>(path: &P) -> Result<Self> {
        DataCore::_create(path.as_ref())
    }
    fn _create(path: &Path) -> Result<Self> {
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
