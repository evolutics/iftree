# Iftree: Include File Tree

Include file data from many files in your Rust code for self-contained binaries.

Self-contained binaries are easy to ship, as they come with any required file
data such as game assets, web templates, etc.

You can think of Iftree as a generalization of `std::include_str!` in two ways:
first, `.gitignore`-like **path patterns** select files from a file tree;
second, files can be associated with **any data** like file contents,
media type, compiled template, etc. Conceptually:

```text
std::include_str!("my_file")
   ↓
   ↓   Iftree
   ↓
any_macros!("my_files/**")
```

## Introduction

Say you have assets in a file tree like

```text
my_assets/
- file_a
- file_b
- subfolder/
  - file_c
```

The generated code allows access to file data as in

```rust
assert_eq!(base::my_assets::FILE_A.content, "… contents of `file_a`\n");
assert_eq!(base::my_assets::FILE_B.content, "… contents of `file_b`\n");
assert_eq!(base::my_assets::subfolder::FILE_C.content, "… and of `file_c`\n");
assert_eq!(ASSETS.len(), 3);
assert_eq!(ASSETS[0].content, "… contents of `file_a`\n");
```

As you can see, access happens via variables `base::path::to::MY_FILE` or via
the `ASSETS` array.

For this to work, you attach the macro `iftree::include_file_tree` to a custom
type as in

```rust
#[iftree::include_file_tree("paths = '/my_assets/**'")]
pub struct MyAsset {
    content: &'static str,
}
```

We just configure a path pattern that filters the files to include, in this case
the files in `my_assets` and its subfolders. These paths are relative to the
folder with your manifest (`Cargo.toml`) by default. For each filtered file, an
instance of `MyAsset` is initialized. Here the well-known field `content` is
initialized with a call to `include_str!`, but you can plug in your own macros.

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

**Field templates** are applied to initialize fields. The standard case is to
include the file contents as code. Among other predefined templates there is one
that includes the file contents only in release builds, while in debug builds it
reads a file afresh on each access. See the
[`field_templates` configuration](#field_templates) for more.

**Custom field templates** enable plugging in your own macros to initialize
fields. With this, you could add file metadata like media types, compress a file
when including it, etc.

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

### `identifiers`

**Default:** `true`

**Examples:**
[`basic_main`](https://github.com/evolutics/iftree/blob/main/examples/basic_main.rs),
[`configuration_identifiers`](https://github.com/evolutics/iftree/blob/main/examples/configuration_identifiers.rs)

### `debug`

**Default:** `false`

**Example:**
[`configuration_debug`](https://github.com/evolutics/iftree/blob/main/examples/configuration_debug.rs)

### `field_templates`

**Examples:**
[`configuration_field_templates_predefined`](https://github.com/evolutics/iftree/blob/main/examples/configuration_field_templates_predefined.rs),
[`configuration_field_templates_custom`](https://github.com/evolutics/iftree/blob/main/examples/configuration_field_templates_custom.rs)
