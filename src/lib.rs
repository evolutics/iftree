//! Include file data from many files in your Rust code for self-contained binaries.
//!
//! # Motivation
//!
//! Self-contained binaries are easy to ship, as they come with any required file
//! data such as game assets, web templates, etc.
//!
//! You can think of Iftree as a generalization of `std::include_str!` in two ways:
//! first, `.gitignore`-like **path patterns** select files from a file tree;
//! second, files can be associated with **any data** like file contents,
//! media type, compiled template, etc. Conceptually:
//!
//! ```text
//! std:       include_str!("my_file")
//! Iftree:    any_macro!("my_files/**")
//! ```
//!
//! # Introduction
//!
//! Say you have assets in a file tree like
//!
//! ```text
//! my_assets/
//! - file_a
//! - file_b
//! - subfolder/
//!   - file_c
//! ```
//!
//! The generated code allows access to file data as in
//!
//! ```ignore
//! assert_eq!(base::my_assets::FILE_A.content, "… contents of `file_a`\n");
//! assert_eq!(base::my_assets::FILE_B.content, "… contents of `file_b`\n");
//! assert_eq!(base::my_assets::subfolder::FILE_C.content, "… and of `file_c`\n");
//! assert_eq!(ASSETS.len(), 3);
//! assert_eq!(ASSETS[0].content, "… contents of `file_a`\n");
//! ```
//!
//! As you can see, access happens via variables `base::path::to::MY_FILE` or via
//! the `ASSETS` array.
//!
//! For this to work, you attach the macro `iftree::include_file_tree` to a custom
//! type as in
//!
//! ```ignore
//! #[iftree::include_file_tree("paths = '/my_assets/**'")]
//! pub struct MyAsset {
//!     content: &'static str,
//! }
//! ```
//!
//! We just configure a path pattern that filters the files to include, in this case
//! the files in `my_assets` and its subfolders. These paths are relative to the
//! folder with your manifest (`Cargo.toml`) by default. For each filtered file, an
//! instance of `MyAsset` is initialized. Here the standard field `content` is
//! initialized with a call to `include_str!`, but you can plug in your own macros.
//!
//! # Feature overview
//!
//! There is an
//! [**`examples` folder**](https://github.com/evolutics/iftree/tree/main/examples)
//! with full code examples to demonstrate the following main aspects.
//!
//! The annotated **asset type** (`MyAsset` above) can be a `struct` with any number
//! of fields. Alternatively, it can be a type alias – especially convenient if
//! there is only one field.
//!
//! To **filter files,** path patterns in a `.gitignore`-like format are supported.
//! This is useful to skip hidden files, filter by filename extension, add multiple
//! folders, use a fixed list of files, etc. See the [`paths` configuration](#paths)
//! for more.
//!
//! An **initializer** is applied to instantiate the asset type once for each file.
//! The standard case is to include the file contents as code. Among other standard
//! fields there is one that includes the file contents only in release builds,
//! while in debug builds it reads a file afresh on each access. You can plug in
//! your own macros as initializers. See the
//! [`initializer` configuration](#initializer) for more.
//!
//! # Configuration
//!
//! The `iftree::include_file_tree` macro is configured via a
//! [TOML](https://toml.io) string with the following fields.
//!
//! ## `paths`
//!
//! **Example:**
//! [`configuration_paths`](https://github.com/evolutics/iftree/blob/main/examples/configuration_paths.rs)
//!
//! ## `base_folder`
//!
//! **Default:** `""`
//!
//! **Example:**
//! [`configuration_base_folder`](https://github.com/evolutics/iftree/blob/main/examples/configuration_base_folder.rs)
//!
//! ## `root_folder_variable`
//!
//! **Default:** `"CARGO_MANIFEST_DIR"`
//!
//! ## `initializer`
//!
//! **Examples:**
//! [`configuration_initializer`](https://github.com/evolutics/iftree/blob/main/examples/configuration_initializer.rs)
//!
//! ## `identifiers`
//!
//! **Default:** `true`
//!
//! **Examples:**
//! [`basic`](https://github.com/evolutics/iftree/blob/main/examples/basic.rs),
//! [`configuration_identifiers`](https://github.com/evolutics/iftree/blob/main/examples/configuration_identifiers.rs)
//!
//! ## `debug`
//!
//! **Default:** `false`
//!
//! **Example:**
//! [`configuration_debug`](https://github.com/evolutics/iftree/blob/main/examples/configuration_debug.rs)

mod data;
mod generate_view;
mod go;
mod list_files;
mod model;
mod parse;
mod print;

/// See the [module level documentation](self).
#[proc_macro_attribute]
pub fn include_file_tree(
    parameters: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let configuration = syn::parse_macro_input!(parameters);
    let item2 = proc_macro2::TokenStream::from(item.clone());
    let type_ = syn::parse_macro_input!(item);

    match go::main(configuration, item2, type_) {
        Err(error) => panic!("{}", error),
        Ok(code) => code.into(),
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
        let embedded_description = format!("\n\n{}\n\n", description);

        let actual = include_str!("../README.md").contains(&embedded_description);

        assert!(actual);
    }

    #[test]
    fn module_documentation_corresponds_to_readme() {
        let mut actual = String::from("# Iftree: Include File Tree 🌳\n\n");
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
