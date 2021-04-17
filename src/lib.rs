//! Include file trees in your code, like `include_bytes!` and `include_str!`
//! for path patterns, with support for custom macros.
//!
//! This is useful for self-contained binaries that are easy to ship, as they
//! include any file data such as web templates, game assets, etc.
//!
//! # Introduction
//!
//! Say you have resources in a file tree like
//!
//! ```text
//! my_resources/
//! - file_a
//! - file_b
//! - subfolder/
//!   - file_c
//! ```
//!
//! The generated code allows access to the file contents as in
//!
//! ```ignore
//! assert_eq!(base::my_resources::FILE_A.content, "… contents of `file_a`\n");
//! assert_eq!(base::my_resources::FILE_B.content, "… contents of `file_b`\n");
//! assert_eq!(base::my_resources::subfolder::FILE_C.content, "… contents of `file_c`\n");
//! ```
//!
//! As you can see, folders are mapped to (nested) modules, which are rooted at a
//! top-level module `base`.
//!
//! For this to work, you attach the macro `iftree::include_file_tree` to a custom
//! type as in
//!
//! ```ignore
//! #[iftree::include_file_tree("paths = '/my_resources/**'")]
//! pub struct MyResource {
//!     content: &'static str,
//! }
//! ```
//!
//! We just configure a path pattern that filters the files to include, in this case
//! the files in `my_resources` and its subfolders. These paths are relative to the
//! folder with your manifest (`Cargo.toml`) by default. For each filtered file, an
//! instance of `MyResource` is initialized. Here the well-known field `content` is
//! initialized with a call to `include_str!`, but you can plug in your own macros.
//!
//! # Feature overview
//!
//! There is an
//! [**`examples` folder**](https://github.com/evolutics/iftree/tree/main/examples)
//! with full code examples to demonstrate the following main aspects.
//!
//! The annotated **resource type** (`MyResource` above) can be a `struct` with any
//! number of fields. Alternatively, it can be a type alias – especially convenient
//! if there is only one field.
//!
//! To **filter files,** path patterns in a `.gitignore`-like format are supported.
//! This is useful to skip hidden files, filter by filename extension, add multiple
//! folders, etc. See the `paths` configuration for more.
//!
//! **Field templates** are applied to initialize fields. The standard case is to
//! include the file contents as code. Among other predefined templates there is one
//! that includes the file contents only in release builds, while in debug builds it
//! reads a file afresh on each access. See the `field_templates` configuration for
//! more.
//!
//! **Custom field templates** enable plugging in your own macros to initialize
//! fields. With this, you could add file metadata like media types, compress a file
//! when including it, etc.

mod data;
mod index_files;
mod model;
mod parse;
mod print;

/// See the [module level documentation](self).
#[proc_macro_attribute]
pub fn include_file_tree(
    parameters: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    process(model::Input { parameters, item })
}

fn process(input: model::Input) -> model::Output {
    let parameters = input.parameters;
    let configuration = syn::parse_macro_input!(parameters);

    let item = input.item;
    let item2 = proc_macro2::TokenStream::from(item.clone());
    let type_ = syn::parse_macro_input!(item);

    match index_files::main(configuration, type_).map(|file_index| print::main(item2, file_index)) {
        Err(error) => panic!("{}", error),
        Ok(value) => value.into(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn readme_includes_manifest_description() {
        let manifest = include_str!("../Cargo.toml")
            .parse::<toml::Value>()
            .unwrap();
        let description = manifest["package"]["description"].as_str().unwrap();
        let embedded_description = format!(
            "\n\n{}\n\n",
            description.replace(" `include_str!` ", " `include_str!`\n"),
        );

        let actual = include_str!("../README.md").contains(&embedded_description);

        assert!(actual);
    }

    #[test]
    fn module_documentation_corresponds_to_readme() {
        let mut actual = String::from("# Iftree: Include File Tree\n\n");
        let mut is_empty = true;
        for line in include_str!("lib.rs").lines() {
            match line.strip_prefix("//!") {
                None => break,
                Some(line) => {
                    let line = line.strip_prefix(' ').unwrap_or(line);
                    let line = if is_empty && line.starts_with('#') {
                        format!("#{}", line)
                    } else if line == "```ignore" {
                        String::from("```rust")
                    } else {
                        String::from(line)
                    };
                    actual.push_str(&format!("{}\n", line));
                    is_empty = line.is_empty();
                }
            }
        }
        let actual = actual;

        let expected = include_str!("../README.md");
        assert_eq!(actual, expected);
    }
}
