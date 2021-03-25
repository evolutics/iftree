pub fn main(base: &str, is_valid: &dyn Fn(&str) -> bool) -> String {
    let mut index = 0;
    loop {
        let identifier = format!("{}{}", base, index);
        if is_valid(&identifier) {
            break identifier;
        }
        index += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates() {
        let actual = main("zeta", &|name| name.len() > 5);

        let expected = "zeta10";
        assert_eq!(actual, expected);
    }
}
