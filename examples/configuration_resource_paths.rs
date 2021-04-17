#[iftree::include_file_tree(
    "
paths = '''
/examples/resources/**
!/examples/resources/world/
/README.md
!.*
'''

[field_templates]
_ = 'relative_path'
"
)]
pub type Resource = &'static str;

pub fn main() {
    assert_eq!(
        ARRAY,
        [
            "README.md",
            "examples/resources/configuration/menu.json",
            "examples/resources/configuration/translations.csv",
            "examples/resources/credits.md",
        ],
    );
}
