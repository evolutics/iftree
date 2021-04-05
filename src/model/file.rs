use super::main;
use std::cmp;

impl Eq for main::File {}

impl Ord for main::File {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match self.relative_path.cmp(&other.relative_path) {
            cmp::Ordering::Equal => comparable_resource_term(&self.resource_term)
                .cmp(&comparable_resource_term(&other.resource_term)),
            cmp::Ordering::Greater => cmp::Ordering::Greater,
            cmp::Ordering::Less => cmp::Ordering::Less,
        }
    }
}

fn comparable_resource_term(resource_term: &main::ResourceTerm) -> main::AbstractResource<String> {
    match resource_term {
        main::ResourceTerm::Unit => main::AbstractResource::Unit,

        main::ResourceTerm::TypeAlias(term) => main::AbstractResource::TypeAlias(term.to_string()),

        main::ResourceTerm::NamedFields(fields) => main::AbstractResource::NamedFields(
            fields
                .iter()
                .map(|(name, term)| (name.clone(), term.to_string()))
                .collect(),
        ),

        main::ResourceTerm::TupleFields(terms) => {
            main::AbstractResource::TupleFields(terms.iter().map(|term| term.to_string()).collect())
        }
    }
}

impl PartialEq for main::File {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == cmp::Ordering::Equal
    }
}

impl PartialOrd for main::File {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path;

    #[test]
    fn compares_equal() {
        let one = main::File {
            relative_path: path::PathBuf::from("abc"),
            resource_term: main::ResourceTerm::TypeAlias(quote::quote! { println!("Hi"); }),
        };
        let another = main::File {
            relative_path: path::PathBuf::from("abc"),
            resource_term: main::ResourceTerm::TypeAlias(quote::quote! { println!("Hi"); }),
        };

        let actual = one.cmp(&another);

        assert_eq!(actual, cmp::Ordering::Equal);
    }

    #[test]
    fn compares_greater() {
        let high = main::File {
            relative_path: path::PathBuf::from("b"),
            ..main::stubs::file()
        };
        let low = main::File {
            relative_path: path::PathBuf::from("a"),
            ..main::stubs::file()
        };

        let actual = high.cmp(&low);

        assert_eq!(actual, cmp::Ordering::Greater);
    }

    #[test]
    fn compares_less() {
        let low = main::File {
            relative_path: path::PathBuf::from("a"),
            ..main::stubs::file()
        };
        let high = main::File {
            relative_path: path::PathBuf::from("b"),
            ..main::stubs::file()
        };

        let actual = low.cmp(&high);

        assert_eq!(actual, cmp::Ordering::Less);
    }

    #[test]
    fn gets_equality() {
        let one = main::File {
            relative_path: path::PathBuf::from("abc"),
            resource_term: main::ResourceTerm::TypeAlias(quote::quote! { println!("Hi"); }),
        };
        let another = main::File {
            relative_path: path::PathBuf::from("abc"),
            resource_term: main::ResourceTerm::TypeAlias(quote::quote! { println!("Hi"); }),
        };

        let actual = one == another;

        assert!(actual);
    }

    #[test]
    fn gets_inequality() {
        let one = main::File {
            relative_path: path::PathBuf::from("abc"),
            resource_term: main::ResourceTerm::TypeAlias(quote::quote! { println!("Hi A"); }),
        };
        let another = main::File {
            relative_path: path::PathBuf::from("abc"),
            resource_term: main::ResourceTerm::TypeAlias(quote::quote! { println!("Hi B"); }),
        };

        let actual = one != another;

        assert!(actual);
    }
}
