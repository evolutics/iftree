pub fn main(original: &str, convention: Convention) -> syn::Ident {
    let name = sanitize_by_convention(original, convention);
    let name = sanitize_special_characters(&name);
    let name = sanitize_first_character(name);
    let name = sanitize_special_cases(name);
    quote::format_ident!("r#{}", name)
}

pub enum Convention {
    ScreamingSnakeCase,
    SnakeCase,
}

fn sanitize_by_convention(name: &str, convention: Convention) -> String {
    match convention {
        Convention::ScreamingSnakeCase => name.to_uppercase(),
        Convention::SnakeCase => name.to_lowercase(),
    }
}

fn sanitize_special_characters(name: &str) -> String {
    name.chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '_'
            }
        })
        .collect()
}

fn sanitize_first_character(name: String) -> String {
    match name.chars().next() {
        Some(first_character) if first_character.is_numeric() => format!("_{}", name),
        _ => name,
    }
}

fn sanitize_special_cases(name: String) -> String {
    match name.as_ref() {
        "" => String::from("__"),
        "_" | "crate" | "self" | "Self" | "super" => format!("{}_", name),
        _ => name,
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
    fn handles_convention_of_screaming_snake_case() {
        let actual = main("README.md", Convention::ScreamingSnakeCase);

        let expected = quote::format_ident!("r#README_MD");
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_convention_of_snake_case() {
        let actual = main("README.md", Convention::SnakeCase);

        let expected = quote::format_ident!("r#readme_md");
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_special_characters() {
        let actual = main("A B##C_D±EÅF𝟙G.H", Convention::ScreamingSnakeCase);

        let expected = quote::format_ident!("r#A_B__C_D_E_F_G_H");
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_first_character() {
        let actual = main("2a", Convention::SnakeCase);

        let expected = quote::format_ident!("r#_2a");
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_empty_string() {
        let actual = main("", stubs::convention());

        let expected = quote::format_ident!("r#__");
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_wildcard_pattern() {
        let actual = main("_", stubs::convention());

        let expected = quote::format_ident!("r#__");
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_special_keywords() {
        let actual = main("self", Convention::SnakeCase);

        let expected = quote::format_ident!("r#self_");
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_other_keywords() {
        let actual = main("match", Convention::SnakeCase);

        let expected = quote::format_ident!("r#match");
        assert_eq!(actual, expected);
    }
}
