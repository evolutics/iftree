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

## Usage

See also the
[**`examples` folder**](https://github.com/evolutics/iftree/tree/main/examples)
if you like to explore by example.

### Getting started

1. Add the dependency `iftree = "0.1"` to your manifest (`Cargo.toml`).
1. Define your **asset type.** This is a `struct` with the fields you need per
   file (`MyAsset` in the [introduction](#introduction)). Alternatively, it can
   be a type alias, which may be convenient if you have a exactly one field.
1. Next, **filter files** to be included by annotating your asset type with
   `#[iftree::include_file_tree("paths = '…'")]`. Path patterns in a
   `.gitignore`-like format are supported. This is useful to skip hidden files,
   filter by filename extension, add multiple folders, use a fixed list of
   files, etc. See the [`paths` configuration](#paths) for more.
1. The generated code then uses an **initializer** to instantiate the asset type
   once per file. By default, a field `contents_str` (if any) is populated with
   `include_str!`, a field `contents_bytes` is populated with `include_bytes!`,
   and a couple of other [standard fields](#standard-fields) are recognized.
   However, you can plug in your own macro to fully customize the initialization
   by [configuring an `initializer`](#initializer).

The
[showcase example](https://github.com/evolutics/iftree/blob/main/examples/showcase.rs)
combines these things.

### Standard fields

When you use fields from the following list only, an initializer for your asset
type is generated without further configuration. You can still override these
field names with a [custom `initializer`](#initializer).

- **`contents_bytes:`** `&'static [u8]`

  File contents as a byte array, using
  [`std::include_bytes`](https://doc.rust-lang.org/std/macro.include_bytes.html).

- **`contents_str:`** `&'static str`

  File contents interpreted as a UTF-8 string, using
  [`std::include_str`](https://doc.rust-lang.org/std/macro.include_str.html).

- **`get_bytes:`** `fn() -> std::borrow::Cow<'static, [u8]>`

  In debug builds (that is, when
  [`debug_assertions`](https://doc.rust-lang.org/reference/conditional-compilation.html#debug_assertions)
  is enabled), this function reads the file afresh on each call at runtime. It
  panics if there is any error such as if the file does not exist. This helps
  with faster development, as it avoids rebuilding if asset file contents are
  changed only (note that you still need to rebuild if assets are added,
  renamed, or removed).

  In release builds, it returns the file contents included at compile time,
  using
  [`std::include_bytes`](https://doc.rust-lang.org/std/macro.include_bytes.html).

- **`get_str:`** `fn() -> std::borrow::Cow<'static, str>`

  Same as `get_bytes` but for the file contents interpreted as a UTF-8 string,
  using
  [`std::include_str`](https://doc.rust-lang.org/std/macro.include_str.html).

- **`relative_path:`** `&'static str`

  File path relative to the base folder, which is the folder with your manifest
  (`Cargo.toml`) by default.

See
[example](https://github.com/evolutics/iftree/blob/main/examples/basics_standard_fields.rs).

### Asset lookup

The generated `ASSETS` array is ordered by the relative path strings, in their
Unicode code point order. This means you can do a binary search for dynamic
lookup (see
[example](https://github.com/evolutics/iftree/blob/main/examples/scenario_binary_search.rs)).

If you know the path at compile time, a static lookup is possible via identifier
`base::path::to::MY_FILE` in constant time. For more, see the
[`identifiers` configuration](#identifiers).

### Troubleshooting

To inspect the generated code, there is a [`debug` configuration](#debug).

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
