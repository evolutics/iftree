pub fn main(original: &str, convention: Convention) -> String {
    let identifier = sanitize_by_convention(original, convention);
    let identifier = sanitize_special_characters(&identifier);
    let identifier = sanitize_first_character(identifier);
    let identifier = sanitize_special_cases(identifier);
    return format!("r#{}", identifier);
}

pub enum Convention {
    ScreamingSnakeCase,
    SnakeCase,
}

fn sanitize_by_convention(identifier: &str, convention: Convention) -> String {
    match convention {
        Convention::ScreamingSnakeCase => identifier.to_uppercase(),
        Convention::SnakeCase => identifier.to_lowercase(),
    }
}

fn sanitize_special_characters(identifier: &str) -> String {
    identifier
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '_'
            }
        })
        .collect()
}

fn sanitize_first_character(identifier: String) -> String {
    match identifier.chars().next() {
        Some(first_character) if first_character.is_numeric() => format!("_{}", identifier),
        _ => identifier,
    }
}

fn sanitize_special_cases(identifier: String) -> String {
    match identifier.as_ref() {
        "" => String::from("__"),
        "_" | "crate" | "self" | "Self" | "super" => format!("{}_", identifier),
        _ => identifier,
    }
}

#[cfg(test)]
mod stubs {
    use super::*;

    pub fn convention() -> Convention {
        Convention::ScreamingSnakeCase
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitizes_by_convention_of_screaming_snake_case() {
        let actual = main("README.md", Convention::ScreamingSnakeCase);

        assert_eq!(actual, "r#README_MD");
    }

    #[test]
    fn sanitizes_by_convention_of_snake_case() {
        let actual = main("README.md", Convention::SnakeCase);

        assert_eq!(actual, "r#readme_md");
    }

    #[test]
    fn sanitizes_special_characters() {
        let actual = main("A B##C_D±EÅF𝟙G.H", Convention::ScreamingSnakeCase);

        assert_eq!(actual, "r#A_B__C_D_E_F_G_H");
    }

    #[test]
    fn sanitizes_first_character() {
        let actual = main("2a", Convention::SnakeCase);

        assert_eq!(actual, "r#_2a");
    }

    #[test]
    fn sanitizes_empty_string() {
        let actual = main("", stubs::convention());

        assert_eq!(actual, "r#__");
    }

    #[test]
    fn sanitizes_wildcard_pattern() {
        let actual = main("_", stubs::convention());

        assert_eq!(actual, "r#__");
    }

    #[test]
    fn sanitizes_special_keywords() {
        let actual = main("self", Convention::SnakeCase);

        assert_eq!(actual, "r#self_");
    }

    #[test]
    fn sanitizes_other_keywords() {
        let actual = main("match", Convention::SnakeCase);

        assert_eq!(actual, "r#match");
    }
}
