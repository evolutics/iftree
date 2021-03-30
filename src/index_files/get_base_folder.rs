use crate::model;
use std::env;
use std::path;

pub fn main(
    get_environment_variable: &dyn Fn(&str) -> Result<String, env::VarError>,
) -> model::Result<path::PathBuf> {
    let folder = get_environment_variable("CARGO_MANIFEST_DIR")
        .map_err(model::Error::EnvironmentVariableCargoManifestDir)?;
    Ok(path::PathBuf::from(folder))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_cargo_manifest_dir_it_gets_it() {
        let actual = main(&|name| {
            if name == "CARGO_MANIFEST_DIR" {
                Ok(String::from("/a"))
            } else {
                Err(env::VarError::NotPresent)
            }
        });

        let actual = actual.unwrap();
        let expected = path::PathBuf::from("/a");
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_no_cargo_manifest_dir_it_errs() {
        let actual = main(&|_| Err(env::VarError::NotPresent));

        let actual = actual.is_err();
        assert!(actual);
    }
}
