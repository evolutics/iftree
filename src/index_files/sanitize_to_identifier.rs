#[allow(dead_code)]
pub fn main(original: &str, convention: Convention) -> String {
    match convention {
        Convention::ScreamingSnakeCase => original.to_uppercase(),
        Convention::SnakeCase => original.to_lowercase(),
    }
    .replace(".", "_")
}

#[allow(dead_code)]
pub enum Convention {
    ScreamingSnakeCase,
    SnakeCase,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitizes_to_screaming_snake_case() {
        let actual = main("README.md", Convention::ScreamingSnakeCase);

        assert_eq!(actual, "README_MD");
    }

    #[test]
    fn sanitizes_to_snake_case() {
        let actual = main("README.md", Convention::SnakeCase);

        assert_eq!(actual, "readme_md");
    }
}
