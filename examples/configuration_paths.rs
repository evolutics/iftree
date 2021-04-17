#[iftree::include_file_tree(
    "
paths = '''
/examples/assets/**
!/examples/assets/world/
/README.md
!.*
'''

[field_templates]
_ = 'relative_path'
"
)]
pub type Asset = &'static str;

pub fn main() {
    assert_eq!(
        ASSETS,
        [
            "README.md",
            "examples/assets/configuration/menu.json",
            "examples/assets/configuration/translations.csv",
            "examples/assets/credits.md",
        ],
    );
}
