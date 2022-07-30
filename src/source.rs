pub struct Source {
    pub raw: String,
}

impl Source {
    pub fn new(raw: String) -> Self {
        Self { raw }
    }

    pub fn download_url(&self) -> String {
        if self.reference().is_none() {
            format!(
                "https://api.github.com/repos/{repository}/tarball/",
                repository = self.repository().unwrap()
            )
        } else {
            format!(
                "https://codeload.github.com/{repository}/legacy.tar.gz/refs/heads/{reference}",
                repository = self.repository().unwrap(),
                reference = self.reference().unwrap()
            )
        }
    }

    fn reference(&self) -> Option<&str> {
        self.raw.splitn(2, '@').nth(1)
    }

    fn repository(&self) -> Option<&str> {
        self.raw.splitn(2, '@').next()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_download_url_without_reference() {
        let source = "rust-lang/rust";
        let url = super::Source::new(source.to_string()).download_url();
        assert_eq!(url, "https://api.github.com/repos/rust-lang/rust/tarball/");
    }

    #[test]
    fn test_download_url_with_reference() {
        let source = "rust-lang/rust@master";
        let url = super::Source::new(source.to_string()).download_url();
        assert_eq!(
            url,
            "https://codeload.github.com/rust-lang/rust/legacy.tar.gz/refs/heads/master"
        );
    }
}
