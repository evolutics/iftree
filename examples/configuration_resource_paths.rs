#[files_embedded_as_modules::embed_files_as_modules(
    "
resource_paths = '''
/examples/resources/**
!/examples/resources/world/
/README.md
!.*
'''

generate_array = true

[field_templates]
_ = '{{relative_path}}'
"
)]
pub type Resource = &'static str;

pub fn main() {
    assert_eq!(
        ARRAY,
        [
            &"README.md",
            &"examples/resources/configuration/menu.json",
            &"examples/resources/configuration/translations.csv",
            &"examples/resources/credits.md",
        ],
    );
}
