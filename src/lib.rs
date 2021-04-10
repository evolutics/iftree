//! Include file trees in your code, like `include_bytes!` and `include_str!`,
//! but for arbitrary path patterns and custom file metadata.
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
//! For this to work, you can call the library as in
//!
//! ```ignore
//! #[iftree::include_file_tree("resource_paths = 'my_resources/**'")]
//! pub struct Resource {
//!     content: &'static str,
//! }
//! ```
//!
//! This calls the macro `iftree::include_file_tree` on a custom type `Resource`.
//! The argument defines a path pattern that configures which files to include, in
//! this case the files in the folder `my_resources`, including its subfolders. For
//! each such file, an instance of `Resource` is initialized with the fields given
//! by `Resource`. The well-known field `content` is initialized with a call to
//! `include_str!`, but you can provide your own macros to initialize a field.

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
    let resource_type = syn::parse_macro_input!(item);

    match index_files::main(configuration, resource_type)
        .map(|file_index| print::main(item2, file_index))
    {
        Err(error) => panic!("{}", error),
        Ok(value) => value.into(),
    }
}

#[cfg(test)]
mod tests {
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
