use crate::model;
use std::env;
use std::path;

pub fn main() -> model::Result<path::PathBuf> {
    Ok(path::PathBuf::from(env::var("CARGO_MANIFEST_DIR")?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets() {
        let actual = main();

        let actual = actual.unwrap();
        let expected = env::current_dir().unwrap();
        assert_eq!(actual, expected);
    }
}
