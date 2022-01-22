pub fn main(original: &str, convention: Convention) -> syn::Ident {
    let name = sanitize_by_convention(original, convention);
    let name = sanitize_special_characters(&name);
    let name = sanitize_first_character(name);
    let name = sanitize_special_cases(name);
    quote::format_ident!("r#{name}")
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
            if unicode_xid::UnicodeXID::is_xid_continue(character) {
                character
            } else {
                '_'
            }
        })
        .collect()
}

fn sanitize_first_character(name: String) -> String {
    match name.chars().next() {
        Some(first_character) if unicode_xid::UnicodeXID::is_xid_start(first_character) => name,
        Some('_') => name,
        _ => format!("_{name}"),
    }
}

fn sanitize_special_cases(name: String) -> String {
    match name.as_ref() {
        "_" | "crate" | "self" | "Self" | "super" => format!("{name}_"),
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
        let actual = main("README_ß_ŉ.md", Convention::ScreamingSnakeCase);

        let expected = quote::format_ident!("r#README_SS_ʼN_MD");
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_convention_of_snake_case() {
        let actual = main("README_ß_ŉ.md", Convention::SnakeCase);

        let expected = quote::format_ident!("r#readme_ß_ŉ_md");
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_special_characters() {
        let actual = main("_0 1##2$3±4√5👽6.7", stubs::convention());

        let expected = quote::format_ident!("r#_0_1__2_3_4_5_6_7");
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_non_ascii_identifiers() {
        let actual = main("åb_π_𝟙", Convention::SnakeCase);

        let expected = quote::format_ident!("r#åb_π_𝟙");
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_first_character_if_xid_start() {
        let actual = main("a", Convention::SnakeCase);

        let expected = quote::format_ident!("r#a");
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_first_character_if_underscore() {
        let actual = main("_2", stubs::convention());

        let expected = quote::format_ident!("r#_2");
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_first_character_if_xid_continue_but_not_xid_start() {
        let actual = main("3", stubs::convention());

        let expected = quote::format_ident!("r#_3");
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_first_character_if_not_xid_continue() {
        let actual = main(".4", stubs::convention());

        let expected = quote::format_ident!("r#_4");
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
