use std::path::Path;

use crate::types::Questionaire;

impl Questionaire {
    pub fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        std::fs::read_to_string(path)?.parse()
    }
}
