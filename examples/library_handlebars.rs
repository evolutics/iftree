use std::collections;

#[iftree::include_file_tree("paths = '/examples/assets/**'")]
pub struct Asset {
    relative_path: &'static str,
    contents_str: &'static str,
}

pub fn main() {
    use base::examples::assets;

    let template_processor = get_template_processor();

    let mut data = collections::HashMap::new();
    data.insert(String::from("name"), String::from("Frodo"));

    assert_eq!(
        template_processor
            .render(assets::configuration::TRANSLATIONS_CSV.relative_path, &data)
            .unwrap(),
        "Hi Frodo\n",
    );
}

fn get_template_processor<'a>() -> handlebars::Handlebars<'a> {
    let mut template_processor = handlebars::Handlebars::new();
    for asset in &ASSETS {
        template_processor
            .register_template_string(asset.relative_path, asset.contents_str)
            .unwrap();
    }
    template_processor
}
