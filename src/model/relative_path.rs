use super::main;

impl From<&str> for main::RelativePath {
    fn from(string: &str) -> Self {
        main::RelativePath(String::from(string))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_from_string() {
        let actual = main::RelativePath::from("a/bc");

        let expected = main::RelativePath(String::from("a/bc"));
        assert_eq!(actual, expected);
    }
}
