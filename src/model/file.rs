use super::main;

impl PartialEq for main::File {
    fn eq(&self, other: &Self) -> bool {
        self.relative_path == other.relative_path
            && comparable_resource_term(&self.resource_term)
                == comparable_resource_term(&other.resource_term)
    }
}

fn comparable_resource_term(resource_term: &main::ResourceTerm) -> main::ResourceStructure<String> {
    match resource_term {
        main::ResourceTerm::Unit => main::ResourceStructure::Unit,

        main::ResourceTerm::TypeAlias(term) => main::ResourceStructure::TypeAlias(term.to_string()),

        main::ResourceTerm::NamedFields(fields) => main::ResourceStructure::NamedFields(
            fields
                .iter()
                .map(|(name, term)| (name.clone(), term.to_string()))
                .collect(),
        ),

        main::ResourceTerm::TupleFields(terms) => main::ResourceStructure::TupleFields(
            terms.iter().map(|term| term.to_string()).collect(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_equality() {
        let one = main::File {
            relative_path: main::RelativePath::from("abc"),
            resource_term: main::ResourceTerm::TypeAlias(quote::quote! { println!("Hi"); }),
        };
        let another = main::File {
            relative_path: main::RelativePath::from("abc"),
            resource_term: main::ResourceTerm::TypeAlias(quote::quote! { println!("Hi"); }),
        };

        let actual = one == another;

        assert!(actual);
    }

    #[test]
    fn gets_inequality() {
        let one = main::File {
            relative_path: main::RelativePath::from("abc"),
            resource_term: main::ResourceTerm::TypeAlias(quote::quote! { println!("Hi A"); }),
        };
        let another = main::File {
            relative_path: main::RelativePath::from("abc"),
            resource_term: main::ResourceTerm::TypeAlias(quote::quote! { println!("Hi B"); }),
        };

        let actual = one != another;

        assert!(actual);
    }
}
