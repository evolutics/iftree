//! Include many files in your Rust code for self-contained binaries.
//!
//! **Highlights**:
//!
//! - Include or exclude files with **path patterns**.
//! - File lookups **checked** at compile time are **fast** at runtime (constant
//!   time).
//! - **Customizable**: associate any data with files.
//! - Many [**examples**](https://github.com/evolutics/iftree/tree/main/examples).
//! - See also [**related projects**](#related-work).
//!
//! ![Test](https://github.com/evolutics/iftree/actions/workflows/test.yml/badge.svg)
//! [![crates.io](https://img.shields.io/crates/v/iftree.svg)](https://crates.io/crates/iftree)
//!
//! # Motivation
//!
//! Self-contained binaries are easy to ship, as they come with any required file
//! data such as game assets, web templates, etc.
//!
//! The standard library's `std::include_str!` includes the contents of a given
//! file. Iftree generalizes this in two ways:
//!
//! - Not just one, but many files can be included at once with **path patterns** in
//!   a `.gitignore`-like format. Patterns are flexible: you can include multiple
//!   folders, skip hidden files, filter by filename extension, select a fixed file
//!   list, etc.
//! - Instead of including the file contents only, files can be associated with
//!   **any data** fields such as additional file metadata.
//!
//! Conceptually:
//!
//! ```text
//! std:       include_str!("my_file")
//! Iftree:    any_macro!("my_files/**")
//! ```
//!
//! # Introduction
//!
//! Here is a minimal example that shows the basic functionality.
//!
//! ```ignore
//! // Say you have these files:
//! //
//! //     my_assets/
//! //     ├── file_a
//! //     ├── file_b
//! //     └── folder/
//! //         └── file_c
//!
//! // To include this file tree in your code, define a custom type like so:
//! #[iftree::include_file_tree("paths = '/my_assets/**'")]
//! pub struct MyAsset {
//!     contents_str: &'static str,
//! }
//! // Above we configure a path pattern to filter the files in `my_assets/` and its
//! // subfolders. For each selected file, an instance of `MyAsset` is initialized.
//! // The standard field `contents_str` is automatically populated with a call to
//! // `include_str!`, but you can plug in your own initializer.
//!
//! fn main() {
//!     // Based on this, an array `ASSETS` with the desired file data is generated:
//!     assert_eq!(ASSETS.len(), 3);
//!     assert_eq!(ASSETS[0].contents_str, "… contents file_a\n");
//!     assert_eq!(ASSETS[1].contents_str, "… contents file_b\n");
//!     assert_eq!(ASSETS[2].contents_str, "… file_c\n");
//!
//!     // Also, variables `base::x::y::MY_FILE` are generated (named by file path):
//!     assert_eq!(base::my_assets::FILE_A.contents_str, "… contents file_a\n");
//!     assert_eq!(base::my_assets::FILE_B.contents_str, "… contents file_b\n");
//!     assert_eq!(base::my_assets::folder::FILE_C.contents_str, "… file_c\n");
//! }
//! ```
//!
//! # Usage
//!
//! Now that you have a general idea of the library, learn how to integrate it with
//! your project.
//!
//! ## Getting started
//!
//! 1. Add the **dependency** `iftree = "1.0"` to your manifest (`Cargo.toml`).
//!
//! 1. Define your **asset type**, which is just a custom `struct` or type alias.
//!    Example:
//!
//!    ```rust
//!    pub struct Asset;
//!    ```
//!
//! 1. Next, **filter files** to be included by annotating your asset type. Example:
//!
//!    ```ignore
//!    #[iftree::include_file_tree("paths = '/my_assets/**'")]
//!    pub struct Asset;
//!    ```
//!
//!    The macro argument is a [TOML](https://toml.io) string literal. Its `paths`
//!    option here supports `.gitignore`-like path patterns, with one pattern per
//!    line. These paths are relative to the folder with your manifest by default.
//!    See the [`paths` configuration](#paths) for more.
//!
//! 1. Define the **data fields** of your asset type. Example:
//!
//!    ```ignore
//!    #[iftree::include_file_tree("paths = '/my_assets/**'")]
//!    pub struct Asset {
//!        relative_path: &'static str,
//!        contents_bytes: &'static [u8],
//!    }
//!    ```
//!
//!    When building your project, code is generated that uses an initializer to
//!    instantiate the asset type once per file.
//!
//!    By default, a field `contents_bytes` (if any) is populated with the file
//!    contents as a byte array, a field `contents_str` is populated with the file
//!    contents as a string, and a couple of other
//!    [standard fields](#standard-fields) are recognized.
//!
//!    However, you can fully customize the initialization by
//!    [configuring an initializer](#templateinitializer). For even more control
//!    over code generation, there is the concept of [visitors](#template-visitors).
//!
//! 1. Now you can **access** your included file data via `ASSETS` array or via
//!    `base::x::y::MY_FILE` variables. Example:
//!
//!    ```ignore
//!    println!("Number of assets: {}", ASSETS.len());
//!    assert_eq!(base::my_assets::MY_FILE.relative_path, "my_assets/my_file");
//!    assert_eq!(base::my_assets::MY_FILE.contents_bytes, b"file contents");
//!    ```
//!
//! ## Examples
//!
//! If you like to explore by example, there is an
//! [**`examples` folder**](https://github.com/evolutics/iftree/tree/main/examples).
//! The documentation links to individual examples where helpful.
//!
//! You could get started with the
//! [introductory example](https://github.com/evolutics/iftree/blob/main/examples/basic.rs).
//! For a more complex case, see the
//! [showcase example](https://github.com/evolutics/iftree/blob/main/examples/showcase.rs).
//!
//! Note that some examples need extra dependencies from the `dev-dependencies` of
//! the [manifest](https://github.com/evolutics/iftree/tree/main/Cargo.toml).
//!
//! ## Standard fields
//!
//! When you use a subset of the following fields only, an initializer for your
//! asset type is generated without further configuration. You can still override
//! these field names with a [custom initializer](#templateinitializer).
//!
//! - **`contents_bytes`**: `&'static [u8]`
//!
//!   File contents as a byte array, using
//!   [`std::include_bytes`](https://doc.rust-lang.org/std/macro.include_bytes.html).
//!
//! - **`contents_str`**: `&'static str`
//!
//!   File contents interpreted as a UTF-8 string, using
//!   [`std::include_str`](https://doc.rust-lang.org/std/macro.include_str.html).
//!
//! - **`get_bytes`**: `fn() -> std::borrow::Cow<'static, [u8]>`
//!
//!   In debug builds (that is, when
//!   [`debug_assertions`](https://doc.rust-lang.org/reference/conditional-compilation.html#debug_assertions)
//!   is enabled), this function reads the file afresh on each call at runtime. It
//!   panics if there is any error such as if the file does not exist. This helps
//!   with faster development, as it avoids rebuilding if asset file contents are
//!   changed only (note that you still need to rebuild if assets are added,
//!   renamed, or removed).
//!
//!   In release builds, it returns the file contents included at compile time,
//!   using
//!   [`std::include_bytes`](https://doc.rust-lang.org/std/macro.include_bytes.html).
//!
//! - **`get_str`**: `fn() -> std::borrow::Cow<'static, str>`
//!
//!   Same as `get_bytes` but for the file contents interpreted as a UTF-8 string,
//!   using
//!   [`std::include_str`](https://doc.rust-lang.org/std/macro.include_str.html).
//!
//! - **`relative_path`**: `&'static str`
//!
//!   File path relative to the base folder, which is the folder with your manifest
//!   (`Cargo.toml`) by default. Path components are separated by a slash `/`,
//!   independent of your platform.
//!
//! See
//! [example](https://github.com/evolutics/iftree/blob/main/examples/basics_standard_fields.rs).
//!
//! ## Name sanitization
//!
//! When generating identifiers based on paths, names are sanitized. For example, a
//! filename `404_not_found.md` is sanitized to an identifier `_404_NOT_FOUND_MD`.
//!
//! The sanitization process is designed to generate valid, conventional
//! [Unicode identifiers](https://doc.rust-lang.org/reference/identifiers.html).
//! Essentially, it replaces invalid identifier characters by underscores `"_"` and
//! adjusts the letter case to the context.
//!
//! More precisely, these transformations are applied in order:
//!
//! 1. The case of letters is adjusted to respect naming conventions:
//!    - All lowercase for folders (because they map to module names).
//!    - All uppercase for filenames (because they map to static variables).
//! 1. Characters without the property `XID_Continue` are replaced by `"_"`. The set
//!    of `XID_Continue` characters in ASCII is `[0-9A-Z_a-z]`.
//! 1. If the first character does not belong to `XID_Start` and is not `"_"`, then
//!    `"_"` is prepended. The set of `XID_Start` characters in ASCII is `[A-Za-z]`.
//! 1. If the name is `"_"`, `"crate"`, `"self"`, `"Self"`, or `"super"`, then `"_"`
//!    is appended.
//!
//! Note that non-ASCII identifiers are only supported from Rust 1.53.0. For earlier
//! versions, the sanitization here may generate invalid identifiers if you use
//! non-ASCII paths, in which case you need to manually rename any affected files.
//!
//! ## Portable file paths
//!
//! To prevent issues when developing on different platforms, your file paths should
//! follow these recommendations:
//!
//! - Path components are separated by a slash `/` (even on Windows).
//! - Filenames do not contain backslashes `\` (even on Unix-like systems).
//!
//! ## Troubleshooting
//!
//! To inspect the generated code, there is a [`debug` configuration](#debug).
//!
//! # Recipes
//!
//! Here are example solutions for given problems.
//!
//! ## Kinds of asset types
//!
//! - [Type alias](https://github.com/evolutics/iftree/blob/main/examples/basics_type_alias.rs)
//!   (`type X = …`)
//! - [Struct](https://github.com/evolutics/iftree/blob/main/examples/basics_type_named_fields.rs)
//!   (`struct` with named fields)
//! - [Tuple struct](https://github.com/evolutics/iftree/blob/main/examples/basics_type_tuple_fields.rs)
//!   (`struct` with unnamed fields)
//! - [Unit-like struct](https://github.com/evolutics/iftree/blob/main/examples/basics_type_unit.rs)
//!   (`struct` without field list)
//!
//! ## Integration with other libraries
//!
//! - Compression with [`include_flate`](https://github.com/evolutics/iftree/blob/main/examples/library_include_flate.rs)
//! - File server with [Actix Web](https://github.com/evolutics/iftree/blob/main/examples/library_actix_web.rs)
//! - File server with [Rocket](https://github.com/evolutics/iftree/blob/main/examples/library_rocket.rs)
//! - File server with [Tide](https://github.com/evolutics/iftree/blob/main/examples/library_tide.rs)
//! - File server with [warp](https://github.com/evolutics/iftree/blob/main/examples/library_warp.rs)
//! - Lazy initialization with [`lazy_static`](https://github.com/evolutics/iftree/blob/main/examples/library_lazy_static.rs)
//! - Lazy initialization with [`once_cell`](https://github.com/evolutics/iftree/blob/main/examples/library_once_cell.rs)
//! - Media types with [`mime_guess`](https://github.com/evolutics/iftree/blob/main/examples/library_mime_guess.rs)
//! - Templates with [Handlebars](https://github.com/evolutics/iftree/blob/main/examples/library_handlebars.rs)
//!
//! ## Including file metadata
//!
//! - [Filename](https://github.com/evolutics/iftree/blob/main/examples/scenario_filename.rs)
//! - [Filename extension](https://github.com/evolutics/iftree/blob/main/examples/scenario_filename_extension.rs)
//! - [Media type](https://github.com/evolutics/iftree/blob/main/examples/scenario_media_type.rs)
//!   (formerly MIME type)
//!
//! ## Custom constructions
//!
//! - [Hash map](https://github.com/evolutics/iftree/blob/main/examples/scenario_hash_map.rs)
//! - [Nested hash map](https://github.com/evolutics/iftree/blob/main/examples/scenario_nested_hash_map.rs)
//!
//! # Related work
//!
//! Originally, I've worked on Iftree because I couldn't find a library for this use
//! case: including files from a folder filtered by filename extension. The project
//! has since developed into something more flexible.
//!
//! Here is how I think Iftree compares to related projects for the given criteria.
//!
//! | Project                                                                            | File selection                                      | Included file data       | Data access via                                                                                     |
//! | ---------------------------------------------------------------------------------- | --------------------------------------------------- | ------------------------ | --------------------------------------------------------------------------------------------------- |
//! | [**`include_dir`**](https://github.com/Michael-F-Bryan/include_dir) 0.6            | Single folder                                       | Path, contents           | File path, nested iterators, glob patterns                                                          |
//! | [**`includedir`**](https://github.com/tilpner/includedir) 0.6                      | Multiple files, multiple folders                    | Path, contents           | File path, iterator                                                                                 |
//! | [**Rust Embed**](https://github.com/pyros2097/rust-embed) 6.0                      | Single folder                                       | Path, contents, metadata | File path, iterator                                                                                 |
//! | [**`std::include_bytes`**](https://doc.rust-lang.org/std/macro.include_bytes.html) | Single file                                         | Contents                 | File path                                                                                           |
//! | [**`std::include_str`**](https://doc.rust-lang.org/std/macro.include_str.html)     | Single file                                         | Contents                 | File path                                                                                           |
//! | **Iftree**                                                                         | Multiple files by inclusion-exclusion path patterns | Path, contents, custom   | File path (via `base::x::y::MY_FILE` variables in constant time), iterator (`ASSETS` array), custom |
//!
//! Generally, while Iftree has defaults to address common use cases, it can be
//! customized to support more specific use cases, too (see [recipes](#recipes) for
//! examples).
//!
//! # Configuration reference
//!
//! The `iftree::include_file_tree` macro is configured via a
//! [TOML](https://toml.io) string with the following fields.
//!
//! ## `base_folder`
//!
//! Path patterns are interpreted as relative to this folder.
//!
//! If this path itself is relative, then it is joined to the folder given by the
//! environment variable `CARGO_MANIFEST_DIR`. That is, a relative path `x/y/z` has
//! a full path `[CARGO_MANIFEST_DIR]/[base_folder]/x/y/z`.
//!
//! **Default**: `""`
//!
//! See
//! [example](https://github.com/evolutics/iftree/blob/main/examples/configuration_base_folder.rs).
//!
//! ## `debug`
//!
//! Whether to generate a string variable `DEBUG` with debug information such as the
//! generated code.
//!
//! **Default**: `false`
//!
//! See
//! [example](https://github.com/evolutics/iftree/blob/main/examples/configuration_debug.rs).
//!
//! ## `paths`
//!
//! A string with a path pattern per line to filter files.
//!
//! It works like a `.gitignore` file with inverted meaning:
//!
//! - If the last matching pattern is negated (with `!`), the file is excluded.
//! - If the last matching pattern is not negated, the file is included.
//! - If no pattern matches, the file is excluded.
//!
//! The pattern language is as documented in the
//! [`.gitignore` reference](https://git-scm.com/docs/gitignore), with this
//! difference: you must use `x/y/*` instead of `x/y/` to include files in a folder
//! `x/y/`; to also include subfolders (recursively), use `x/y/**`.
//!
//! Exclude hidden files with `!.*` as a pattern. Another common pattern is of the
//! form `*.xyz` to include files with filename extension `xyz` only.
//!
//! By default, path patterns are relative to the environment variable
//! `CARGO_MANIFEST_DIR`, which is the folder with your manifest (`Cargo.toml`). See
//! the [`base_folder` configuration](#base_folder) to customize this.
//!
//! This is a **required** option without default.
//!
//! See
//! [example](https://github.com/evolutics/iftree/blob/main/examples/configuration_paths.rs).
//!
//! ## `root_folder_variable`
//!
//! The name of the environment variable to use as the root folder for the
//! [`base_folder` configuration](#base_folder).
//!
//! The value of the environment variable should be an absolute path.
//!
//! **Default**: `"CARGO_MANIFEST_DIR"`
//!
//! ## `template.identifiers`
//!
//! Whether to generate an identifier per file.
//!
//! Given a file `x/y/my_file`, a `static` variable `base::x::y::MY_FILE` is
//! generated, nested in modules for folders. Their root module is `base`, which
//! represents the base folder.
//!
//! Each variable is a reference to the corresponding element in the `ASSETS` array.
//!
//! Generated identifiers are subject to [name sanitization](#name-sanitization).
//! Because of this, there may be collisions in the generated code, causing an error
//! about a name being defined multiple times. The code generation does not try to
//! resolve such collisions automatically, as this would likely cause confusion
//! about which identifier refers to which file. Instead, you need to manually
//! rename any affected paths (assuming you need the generated identifiers at all –
//! otherwise, you can just disable this with `template.identifiers = false`).
//!
//! **Default**: `true`
//!
//! See
//! [example](https://github.com/evolutics/iftree/blob/main/examples/configuration_template_identifiers.rs).
//!
//! ## `template.initializer`
//!
//! A macro name used to instantiate the asset type per file.
//!
//! As inputs, the macro is passed the following arguments, separated by comma:
//!
//! 1. Relative file path as a string literal. Path components are separated by `/`.
//! 1. Absolute file path as a string literal.
//!
//! As an output, the macro must return a
//! [constant expression](https://doc.rust-lang.org/reference/const_eval.html#constant-expressions).
//!
//! **Default**: A default initializer is constructed by recognizing
//! [standard fields](#standard-fields).
//!
//! See
//! [example](https://github.com/evolutics/iftree/blob/main/examples/configuration_template_initializer.rs).
//!
//! ## `template` visitors
//!
//! This is the most flexible customization of the code generation process.
//!
//! Essentially, a visitor transforms the tree of selected files into code. It does
//! so by calling custom macros at these levels:
//!
//! - For the base folder, a `visit_base` macro is called to wrap everything (top
//!   level).
//! - For each folder, a `visit_folder` macro is called, wrapping the code generated
//!   from its files and subfolders (recursively).
//! - For each file, a `visit_file` macro is called (bottom level).
//!
//! These macros are passed the following inputs, separated by comma:
//!
//! - `visit_base`:
//!   1. Total number of selected files as a `usize` literal.
//!   1. Outputs of the visitor applied to the base folder entries, ordered by
//!      filename in Unicode code point order.
//! - `visit_folder`:
//!   1. Folder name as a string literal.
//!   1. [Sanitized](#name-sanitization) folder name as an identifier.
//!   1. Outputs of the visitor applied to the folder entries, ordered by filename
//!      in Unicode code point order.
//! - `visit_file`:
//!   1. Filename as a string literal.
//!   1. [Sanitized](#name-sanitization) filename as an identifier.
//!   1. Zero-based index of the file among the selected files as a `usize` literal.
//!   1. Relative file path as a string literal. Path components are separated by
//!      `/`.
//!   1. Absolute file path as a string literal.
//!
//! The `visit_folder` macro is optional. If missing, the outputs of the
//! `visit_file` calls are directly passed as an input to the `visit_base` call.
//! This is useful to generate flat structures such as arrays. Similarly, the
//! `visit_base` macro is optional.
//!
//! You can configure multiple visitors. They are applied in order.
//!
//! To plug in visitors, add this to your configuration for each visitor:
//!
//! ```toml
//! [[template]]
//! visit_base = 'visit_my_base'
//! visit_folder = 'visit_my_folder'
//! visit_file = 'visit_my_file'
//!
//! ```
//!
//! `visit_my_…` are the names of your corresponding macros.
//!
//! See examples:
//!
//! - [Basic](https://github.com/evolutics/iftree/blob/main/examples/configuration_template_visitors.rs)
//! - [Nesting](https://github.com/evolutics/iftree/blob/main/examples/configuration_template_visitors_nesting.rs)
//! - [Emulation of default code generation](https://github.com/evolutics/iftree/blob/main/examples/configuration_template_visitors_emulation.rs)
//!
//! # Further resources
//!
//! - [Changelog](https://github.com/evolutics/iftree/blob/main/CHANGELOG.md)
//! - [crates.io](https://crates.io/crates/iftree)

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
    use std::fs;

    #[test]
    fn readme_includes_manifest_description() {
        let embedded_description = format!("\n\n{}\n\n", env!("CARGO_PKG_DESCRIPTION"));

        let actual = get_readme().contains(&embedded_description);

        assert!(actual);
    }

    fn get_readme() -> &'static str {
        include_str!("../README.md")
    }

    #[test]
    fn readme_includes_basic_example() {
        let example = format!("```rust\n{}```\n", include_str!("../examples/basic.rs"));

        let actual = get_readme().contains(&example);

        assert!(actual);
    }

    #[test]
    fn readme_refers_to_current_manifest_version() {
        let dependency = format!(
            "`{} = \"{}.{}\"`",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION_MAJOR"),
            env!("CARGO_PKG_VERSION_MINOR"),
        );

        let actual = get_readme().contains(&dependency);

        assert!(actual);
    }

    #[test]
    fn readme_links_to_each_example_exactly_once() {
        let mut actual =
            regex::Regex::new(r"https://github.com/evolutics/iftree/blob/main/examples/([^)]+)")
                .unwrap()
                .captures_iter(get_readme())
                .map(|captures| captures.get(1).unwrap().as_str())
                .collect::<Vec<_>>();

        actual.sort_unstable();
        let actual = actual;
        let mut expected = fs::read_dir("examples")
            .unwrap()
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .filter(|filename| filename.ends_with(".rs"))
            .collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }

    #[test]
    fn changelog_contains_current_manifest_version() {
        let version_section = format!("\n\n## {} – ", env!("CARGO_PKG_VERSION"));

        let actual = get_changelog().contains(&version_section);

        assert!(actual);
    }

    fn get_changelog() -> &'static str {
        include_str!("../CHANGELOG.md")
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
                    } else {
                        line.replace("```ignore", "```rust")
                    };
                    actual.push_str(&format!("{}\n", line));
                    is_empty = line.is_empty();
                }
            }
        }
        let actual = actual;

        let expected = get_readme();
        assert_eq!(actual, expected);
    }
}
