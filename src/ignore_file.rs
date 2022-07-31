const IGNORE_FILE_NAME: &str = ".gitcpignore";

pub struct IgnoreFile {
    path: std::path::PathBuf,
}

impl IgnoreFile {
    pub fn find(mut paths: impl Iterator<Item = std::path::PathBuf>) -> Option<Self> {
        let pattern = format!("*/{}", IGNORE_FILE_NAME);
        let glob = globset::Glob::new(&pattern).unwrap().compile_matcher();
        paths
            .find(|path| glob.is_match(path.to_str().unwrap()))
            .map(|path| IgnoreFile { path })
    }

    pub fn to_glob_set(&self) -> globset::GlobSet {
        let mut builder = globset::GlobSetBuilder::new();
        let pattern = format!("*/{}", IGNORE_FILE_NAME);
        builder.add(globset::Glob::new(&pattern).unwrap());
        for item in self.items() {
            let pattern = format!("*/{}", item);
            builder.add(globset::Glob::new(&pattern).unwrap());
        }
        builder.build().unwrap()
    }

    fn content(&self) -> std::io::Result<String> {
        std::fs::read_to_string(self.path.to_str().unwrap())
    }

    fn items(&self) -> Vec<String> {
        self.content()
            .unwrap()
            .lines()
            .map(|line| line.split('#').next().unwrap().trim())
            .filter(|line| !line.is_empty())
            .map(|line| line.to_string())
            .collect()
    }
}
