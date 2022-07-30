pub struct Source {
    pub raw: String,
}

impl Source {
    pub fn new(raw: String) -> Self {
        Self { raw }
    }

    pub fn download_url(&self) -> Option<String> {
        if self.reference().is_none() {
            Some(format!(
                "https://api.github.com/repos/{repository_identifier}/tarball/",
                repository_identifier = self.repository_identifier()?
            ))
        } else {
            Some(format!(
                "https://codeload.github.com/{repository_identifier}/legacy.tar.gz/refs/heads/{reference}",
                repository_identifier = self.repository_identifier()?,
                reference = self.reference()?
            ))
        }
    }

    pub fn glob_pattern(&self) -> String {
        format!("*/{}", self.file_path().unwrap_or_else(|| "*".to_string()))
    }

    fn reference(&self) -> Option<String> {
        self.raw.split_once('@').map(|x| x.1.to_string())
    }

    fn repository_identifier(&self) -> Option<String> {
        let mut segments = self.raw.split('@').next()?.splitn(3, '/');
        let owner = segments.next()?;
        let name = segments.next()?;
        Some(format!("{owner}/{name}", owner = owner, name = name))
    }

    fn file_path(&self) -> Option<String> {
        self.raw
            .split('@')
            .next()?
            .splitn(3, '/')
            .nth(2)
            .map(|x| x.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_download_url_without_reference() {
        let source = "rust-lang/rust";
        let url = super::Source::new(source.to_string())
            .download_url()
            .unwrap();
        assert_eq!("https://api.github.com/repos/rust-lang/rust/tarball/", url);
    }

    #[test]
    fn test_download_url_with_reference() {
        let source = "rust-lang/rust@master";
        let url = super::Source::new(source.to_string())
            .download_url()
            .unwrap();
        assert_eq!(
            "https://codeload.github.com/rust-lang/rust/legacy.tar.gz/refs/heads/master",
            url
        );
    }

    #[test]
    fn test_glob_pattern_on_none_case() {
        let source = "rust-lang/rust";
        let glob_pattern = super::Source::new(source.to_string()).glob_pattern();
        assert_eq!("*/*", glob_pattern);
    }

    #[test]
    fn test_glob_pattern_on_some_case() {
        let source = "rust-lang/rust/foo/bar";
        let glob_pattern = super::Source::new(source.to_string()).glob_pattern();
        assert_eq!("*/foo/bar", glob_pattern);
    }
}
