macro_rules! visit_base {
    ($length:literal, $($contents:item)*) => {
        pub mod base {
            $($contents)*
        }
    };
}

macro_rules! visit_folder {
    ($name:literal, $id:ident, $($contents:item)*) => {
        pub mod $id {
            $($contents)*
        }
    };
}

macro_rules! visit_file {
    ($name:literal, $id:ident, $index:literal, $relative_path:literal, $absolute_path:literal) => {
        include_flate::flate!(pub static $id: str from $absolute_path);
    };
}

#[iftree::include_file_tree(
    "
paths = '/examples/assets/**'

[[template]]
visit_base = 'visit_base'
visit_folder = 'visit_folder'
visit_file = 'visit_file'
"
)]
pub struct Asset;

pub fn main() {
    use base::examples::assets;

    assert_eq!(*assets::CREDITS_MD, "Boo Far\n");
}
