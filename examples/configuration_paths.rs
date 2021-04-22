macro_rules! initialize {
    ($relative_path:literal, $absolute_path:literal) => {
        $relative_path
    };
}

#[iftree::include_file_tree(
    "
paths = '''
/examples/assets/**
!/examples/assets/world/
/README.md
!.*
'''

initializer = 'initialize'
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
