pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The default source file written for a new package.
pub const DEFAULT_MAIN: &str = "fn main() {\n    println!(\"Hello, craft!\");\n}\n";

/// Validate a package name: lowercase letters, digits, dashes, and underscores.
pub fn valid_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
}

/// Render the manifest for a new craft package.
pub fn manifest_contents(name: &str) -> String {
    format!("name = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_names() {
        assert!(valid_name("foo"));
        assert!(valid_name("foo-bar"));
        assert!(valid_name("foo_2"));
    }

    #[test]
    fn rejects_invalid_names() {
        assert!(!valid_name(""));
        assert!(!valid_name("Foo"));
        assert!(!valid_name("foo bar"));
        assert!(!valid_name("foo/bar"));
    }

    #[test]
    fn renders_manifest() {
        let manifest = manifest_contents("my-tool");
        assert_eq!(
            manifest,
            "name = \"my-tool\"\nversion = \"0.1.0\"\nedition = \"2021\"\n"
        );
    }
}
