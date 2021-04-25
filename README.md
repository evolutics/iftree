# Iftree: Include File Tree 🌳

Include file data from many files in your Rust code for self-contained binaries.

## Motivation

Self-contained binaries are easy to ship, as they come with any required file
data such as game assets, web templates, etc.

You can think of Iftree as a generalization of `std::include_str!` in two ways:

1. Instead of including a single file, many files can be included with
   **path patterns** in a `.gitignore`-like format.
1. Instead of including the file contents only, files can be associated with
   **any data** such as their media type, a compiled template, etc.

Conceptually:

```text
std:       include_str!("my_file")
Iftree:    any_macro!("my_files/**")
```

## Introduction

Let's explore the basic functionality with a first example.

Say you have the following files in a folder `my_assets`:

```text
my_assets/
- file_a
- file_b
- subfolder/
  - file_c
```

To include data from these files in your code, Iftree generates an array
`ASSETS` with an element per included file:

```rust
assert_eq!(ASSETS.len(), 3);
assert_eq!(ASSETS[0].contents_str, "… contents `file_a`\n");
assert_eq!(ASSETS[1].contents_str, "… contents `file_b`\n");
assert_eq!(ASSETS[2].contents_str, "… and `file_c`\n");
```

Furthermore, variables `base::path::to::MY_FILE` are generated:

```rust
assert_eq!(base::my_assets::FILE_A.contents_str, "… contents `file_a`\n");
assert_eq!(base::my_assets::FILE_B.contents_str, "… contents `file_b`\n");
assert_eq!(base::my_assets::subfolder::FILE_C.contents_str, "… and `file_c`\n");
```

For this to work, you just attach the macro `iftree::include_file_tree` to a
custom type like so:

```rust
#[iftree::include_file_tree("paths = '/my_assets/**'")]
pub struct MyAsset {
    contents_str: &'static str,
}
```

Here we configure a path pattern that filters the files to include, in this case
the files in `my_assets` and its subfolders. These paths are relative to the
folder with your manifest (`Cargo.toml`) by default. For each selected file, an
instance of `MyAsset` is initialized. The standard field `contents_str` is
automatically populated with a call to `include_str!`, but you can plug in your
own initializer.

## Feature overview

There is an
[**`examples` folder**](https://github.com/evolutics/iftree/tree/main/examples)
with full code examples to demonstrate the following main aspects.

The annotated **asset type** (`MyAsset` above) can be a `struct` with any number
of fields. Alternatively, it can be a type alias – especially convenient if
there is only one field.

To **filter files,** path patterns in a `.gitignore`-like format are supported.
This is useful to skip hidden files, filter by filename extension, add multiple
folders, use a fixed list of files, etc. See the [`paths` configuration](#paths)
for more.

An **initializer** is applied to instantiate the asset type once for each file.
The standard case is to include the file contents as code. Among other standard
fields there is one that includes the file contents only in release builds,
while in debug builds it reads a file afresh on each access. You can plug in
your own macros as initializers. See the
[`initializer` configuration](#initializer) for more.

## Configuration

The `iftree::include_file_tree` macro is configured via a
[TOML](https://toml.io) string with the following fields.

### `paths`

**Example:**
[`configuration_paths`](https://github.com/evolutics/iftree/blob/main/examples/configuration_paths.rs)

### `base_folder`

**Default:** `""`

**Example:**
[`configuration_base_folder`](https://github.com/evolutics/iftree/blob/main/examples/configuration_base_folder.rs)

### `root_folder_variable`

**Default:** `"CARGO_MANIFEST_DIR"`

### `initializer`

**Examples:**
[`configuration_initializer`](https://github.com/evolutics/iftree/blob/main/examples/configuration_initializer.rs)

### `identifiers`

**Default:** `true`

**Examples:**
[`basic`](https://github.com/evolutics/iftree/blob/main/examples/basic.rs),
[`configuration_identifiers`](https://github.com/evolutics/iftree/blob/main/examples/configuration_identifiers.rs)

### `debug`

**Default:** `false`

**Example:**
[`configuration_debug`](https://github.com/evolutics/iftree/blob/main/examples/configuration_debug.rs)
