use super::main;

impl From<&str> for main::RelativePath {
    fn from(string: &str) -> Self {
        main::RelativePath(String::from(string))
    }
}
