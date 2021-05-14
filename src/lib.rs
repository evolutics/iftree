//! Include file data from many files in your Rust code for self-contained binaries.
//!
//! ![test](https://github.com/evolutics/iftree/actions/workflows/test.yml/badge.svg)
//!
//! # Motivation
//!
//! Self-contained binaries are easy to ship, as they come with any required file
//! data such as game assets, web templates, etc.
//!
//! The standard library's `std::include_str!` includes the contents of a given
//! file. Iftree generalizes this in two ways:
//!
//! 1. Not just one, but many files can be included at once with **path patterns**
//!    in a `.gitignore`-like format. Patterns are flexible: you can skip hidden
//!    files, filter by filename extension, select a fixed list of files, etc.
//! 1. Instead of including the file contents only, files can be associated with
//!    **any data** fields such as additional file metadata.
//!
//! Conceptually:
//!
//! ```text
//! std:       include_str!("my_file")
//! Iftree:    any_macro!("my_files/**")
//! ```
//!
//! Refer to [**related work**](#related-work) to see Iftree in the context of
//! other, similar projects.
//!
//! # Introduction
//!
//! Here is a minimal example that shows the basic functionality.
//!
//! ```ignore
//! // Say you have the following files:
//! //
//! //     my_assets/
//! //     ├── file_a
//! //     ├── file_b
//! //     └── folder/
//! //         └── file_c
//!
//! // To include these files in your code, the macro `iftree::include_file_tree` is
//! // attached to a custom type like this:
//! #[iftree::include_file_tree("paths = '/my_assets/**'")]
//! pub struct MyAsset {
//!     contents_str: &'static str,
//! }
//! // Above we configure a path pattern to filter the files in `my_assets/` and its
//! // subfolders. For each selected file, an instance of `MyAsset` is initialized.
//! // The standard field `contents_str` is automatically populated with a call to
//! // `include_str!`, but you can plug in your own initializer.
//!
//! pub fn main() {
//!     // Based on this, Iftree generates an array `ASSETS` with the desired file
//!     // data. You can use it like so:
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
//! 1. Add the **dependency** `iftree = "0.1"` to your manifest (`Cargo.toml`).
//!
//! 1. Define your **asset type** (`MyAsset` in the [introduction](#introduction)).
//!
//!    This is a `struct` with the fields you need per file. Alternatively, it can
//!    be a type alias, which may be convenient if you have exactly one field.
//!
//! 1. Next, **filter files** to be included by annotating your asset type with
//!    `#[iftree::include_file_tree("paths = '/my/assets/**'")]`.
//!
//!    The macro argument is a [TOML](https://toml.io) string literal. Its `paths`
//!    option here supports `.gitignore`-like path patterns, with one pattern per
//!    line. These paths are relative to the folder with your manifest by default.
//!    See the [`paths` configuration](#paths) for more.
//!
//! 1. When building your project, code is generated that uses an **initializer** to
//!    instantiate the asset type once per file.
//!
//!    By default, a field `contents_str` (if any) is populated with `include_str!`,
//!    a field `contents_bytes` is populated with `include_bytes!`, and a couple of
//!    other [standard fields](#standard-fields) are recognized. However, you can
//!    plug in your own macro to fully customize the initialization by
//!    [configuring an initializer](#templateinitializer). For even more control
//!    over code generation, there is the concept of [visitors](#template-visitors).
//!
//! 1. Now you can **access** your included file data via `ASSETS` array or via
//!    `base::my::assets::MY_FILE` variables.
//!
//! ## Examples
//!
//! If you like to explore by example, there is an
//! [**`examples` folder**](https://github.com/evolutics/iftree/tree/main/examples).
//! The documentation links to individual examples where helpful.
//!
//! Note that some examples need extra dependencies from the `dev-dependencies` of
//! the [manifest](https://github.com/evolutics/iftree/tree/main/Cargo.toml).
//!
//! ## Standard fields
//!
//! When you use fields from the following list only, an initializer for your asset
//! type is generated without further configuration. You can still override these
//! field names with a [custom initializer](#templateinitializer).
//!
//! - **`contents_bytes:`** `&'static [u8]`
//!
//!   File contents as a byte array, using
//!   [`std::include_bytes`](https://doc.rust-lang.org/std/macro.include_bytes.html).
//!
//! - **`contents_str:`** `&'static str`
//!
//!   File contents interpreted as a UTF-8 string, using
//!   [`std::include_str`](https://doc.rust-lang.org/std/macro.include_str.html).
//!
//! - **`get_bytes:`** `fn() -> std::borrow::Cow<'static, [u8]>`
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
//! - **`get_str:`** `fn() -> std::borrow::Cow<'static, str>`
//!
//!   Same as `get_bytes` but for the file contents interpreted as a UTF-8 string,
//!   using
//!   [`std::include_str`](https://doc.rust-lang.org/std/macro.include_str.html).
//!
//! - **`relative_path:`** `&'static str`
//!
//!   File path relative to the base folder, which is the folder with your manifest
//!   (`Cargo.toml`) by default.
//!
//! See
//! [example](https://github.com/evolutics/iftree/blob/main/examples/basics_standard_fields.rs).
//!
//! ## Name sanitization
//!
//! When generating identifiers based on paths, names are sanitized as follows to
//! ensure they are
//! [valid identifiers](https://doc.rust-lang.org/reference/identifiers.html):
//!
//! - Characters other than ASCII alphanumericals are replaced by `"_"`.
//! - If the first character is numeric, then `"_"` is prepended.
//! - If the name is `"_"`, `"crate"`, `"self"`, `"Self"`, or `"super"`, then `"_"`
//!   is appended.
//!
//! Names are further adjusted to respect naming conventions in the default case:
//!
//! - Lowercase for folders (because they map to module names).
//! - Uppercase for filenames (because they map to static variables).
//!
//! ## Portable file paths
//!
//! To prevent issues when developing on different platforms, any paths in your
//! configuration should follow these recommendations:
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
//! - File server with [Tide](https://github.com/evolutics/iftree/blob/main/examples/library_tide.rs)
//! - File server with [warp](https://github.com/evolutics/iftree/blob/main/examples/library_warp.rs)
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
//! | Project                                                                            | File selection                                      | Included file data     | Data access via                                                         |
//! | ---------------------------------------------------------------------------------- | --------------------------------------------------- | ---------------------- | ----------------------------------------------------------------------- |
//! | [**`include_dir`**](https://github.com/Michael-F-Bryan/include_dir) 0.6            | Single folder                                       | Path, contents         | File path, nested iterators, glob patterns                              |
//! | [**`includedir`**](https://github.com/tilpner/includedir) 0.6                      | Multiple files, multiple folders                    | Path, contents         | File path, iterator                                                     |
//! | [**Rust Embed**](https://github.com/pyros2097/rust-embed) 5.9                      | Single folder                                       | Path, contents         | File path, iterator                                                     |
//! | [**`std::include_bytes`**](https://doc.rust-lang.org/std/macro.include_bytes.html) | Single file                                         | Byte contents          | Literal file path                                                       |
//! | [**`std::include_str`**](https://doc.rust-lang.org/std/macro.include_str.html)     | Single file                                         | String contents        | Literal file path                                                       |
//! | **Iftree**                                                                         | Multiple files by inclusion-exclusion path patterns | Path, contents, custom | `base::x::y::MY_FILE` variables (constant time), `ASSETS` array, custom |
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
//! **Default:** `""`
//!
//! See
//! [example](https://github.com/evolutics/iftree/blob/main/examples/configuration_base_folder.rs).
//!
//! ## `debug`
//!
//! Whether to generate a string variable `DEBUG` with debug information such as the
//! generated code.
//!
//! **Default:** `false`
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
//! **Default:** `"CARGO_MANIFEST_DIR"`
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
//! rename any affected paths.
//!
//! **Default:** `true`
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
//! 1. Relative file path as a string literal.
//! 1. Absolute file path as a string literal.
//!
//! As an output, the macro must return a
//! [constant expression](https://doc.rust-lang.org/reference/const_eval.html#constant-expressions).
//!
//! **Default:** A default initializer is constructed by recognizing
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
//!   1. Outputs of the visitor applied to the base folder entries.
//! - `visit_folder`:
//!   1. Folder name as a string literal.
//!   1. [Sanitized](#name-sanitization) folder name as an identifier.
//!   1. Outputs of the visitor applied to the folder entries.
//! - `visit_file`:
//!   1. Filename as a string literal.
//!   1. [Sanitized](#name-sanitization) filename as an identifier.
//!   1. Zero-based index of the file among the selected files as a `usize` literal.
//!   1. Relative file path as a string literal.
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
//! See
//! [example](https://github.com/evolutics/iftree/blob/main/examples/configuration_template_visitors.rs).

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
        let manifest = get_manifest();
        let description = manifest["package"]["description"].as_str().unwrap();
        let embedded_description = format!("\n\n{}\n\n", description);

        let actual = get_readme().contains(&embedded_description);

        assert!(actual);
    }

    fn get_manifest() -> toml::Value {
        include_str!("../Cargo.toml").parse().unwrap()
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
        let manifest = get_manifest();
        let version = manifest["package"]["version"].as_str().unwrap();
        let major_minor_version = version.rsplitn(2, '.').nth(1).unwrap();
        let dependency = format!("`iftree = \"{}\"`", major_minor_version);

        let actual = get_readme().contains(&dependency);

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

        let expected = get_readme();
        assert_eq!(actual, expected);
    }
}
