# Iftree: Include File Tree

Include file trees in your code, like `include_bytes!` and `include_str!`,
but for arbitrary path patterns and custom file metadata.

This is useful for self-contained binaries that are easy to ship, as they
include any file data such as web templates, game assets, etc.

## Introduction

Say you have resources in a file tree like

```text
my_resources/
- file_a
- file_b
- subfolder/
  - file_c
```

The generated code allows access to the file contents as in

```rust
assert_eq!(base::my_resources::FILE_A.content, "… contents of `file_a`\n");
assert_eq!(base::my_resources::FILE_B.content, "… contents of `file_b`\n");
assert_eq!(base::my_resources::subfolder::FILE_C.content, "… contents of `file_c`\n");
```

For this to work, you can call the library as in

```rust
#[iftree::include_file_tree("resource_paths = 'my_resources/**'")]
pub struct Resource {
    content: &'static str,
}
```

This calls the macro `iftree::include_file_tree` on a custom type `Resource`.
The argument defines a path pattern that configures which files to include, in
this case the files in the folder `my_resources`, including its subfolders. For
each such file, an instance of `Resource` is initialized with the fields given
by `Resource`. The well-known field `content` is initialized with a call to
`include_str!`, but you can provide your own macros to initialize a field.
