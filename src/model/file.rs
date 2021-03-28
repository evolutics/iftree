use super::main;
use std::cmp;

impl Ord for main::File {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.relative_path.cmp(&other.relative_path)
    }
}

impl PartialEq for main::File {
    fn eq(&self, other: &Self) -> bool {
        self.relative_path == other.relative_path
    }
}

impl PartialOrd for main::File {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
