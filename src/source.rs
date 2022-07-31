use crate::error::Error;

#[derive(Debug)]
pub struct Source {
    pub download_url: String,
    pub glob_pattern: String,
}

impl Source {
    pub fn parse(raw: impl Into<String>) -> crate::result::Result<Self> {
        let raw = raw.into();
        let mut sections = raw.split('@');
        let body = sections.next().unwrap();
        let reference = sections.next();

        let mut parts = body.splitn(3, '/');

        let owner = parts.next().unwrap();
        let name = parts.next().ok_or(Error::InvalidSourceError {
            source: owner.to_string(),
        })?;
        let file_path = parts.next();

        let download_url = format!(
            "https://api.github.com/repos/{owner}/{name}/tarball/{reference}",
            name = name,
            owner = owner,
            reference = reference.unwrap_or(""),
        );

        let glob_pattern = format!("*/{}", file_path.unwrap_or("*"));

        Ok(Source {
            download_url,
            glob_pattern,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse() {
        let raw = "rust-lang/rust";
        let source = super::Source::parse(raw).unwrap();
        assert_eq!(
            "https://api.github.com/repos/rust-lang/rust/tarball/",
            source.download_url
        );
        assert_eq!("*/*", source.glob_pattern);
    }

    #[test]
    fn test_parse_with_file_path() {
        let raw = "rust-lang/rust/README.md";
        let source = super::Source::parse(raw).unwrap();
        assert_eq!(
            "https://api.github.com/repos/rust-lang/rust/tarball/",
            source.download_url
        );
        assert_eq!("*/README.md", source.glob_pattern);
    }

    #[test]
    fn test_parse_with_reference() {
        let raw = "rust-lang/rust@main";
        let source = super::Source::parse(raw).unwrap();
        assert_eq!(
            "https://api.github.com/repos/rust-lang/rust/tarball/main",
            source.download_url
        );
        assert_eq!("*/*", source.glob_pattern);
    }

    #[test]
    fn test_parse_with_file_path_and_reference() {
        let raw = "rust-lang/rust/README.md@main";
        let source = super::Source::parse(raw).unwrap();
        assert_eq!(
            "https://api.github.com/repos/rust-lang/rust/tarball/main",
            source.download_url
        );
        assert_eq!("*/README.md", source.glob_pattern);
    }
}
