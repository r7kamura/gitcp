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
                "https://api.github.com/repos/{repository}/tarball/",
                repository = self.repository()?
            ))
        } else {
            Some(format!(
                "https://codeload.github.com/{repository}/legacy.tar.gz/refs/heads/{reference}",
                repository = self.repository()?,
                reference = self.reference()?
            ))
        }
    }

    fn reference(&self) -> Option<&str> {
        self.raw.split_once('@').map(|x| x.1)
    }

    fn repository(&self) -> Option<&str> {
        self.raw.split('@').next()
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
}
