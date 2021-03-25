use crate::model;
use std::env;
use std::path;

pub fn main(configuration: &model::Configuration) -> model::Result<path::PathBuf> {
    env::var("CARGO_MANIFEST_DIR")
        .map_err(model::Error::EnvironmentVariableCargoManifestDir)
        .map(|folder| path::PathBuf::from(folder).join(&configuration.resource_folder))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets() {
        let actual = main(&model::Configuration {
            resource_folder: path::PathBuf::from("example/resources"),
        })
        .unwrap();

        assert!(actual.ends_with("example/resources"));
    }
}
