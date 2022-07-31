// For `Opt::from_args`.
use structopt::StructOpt;

#[tokio::main]
async fn main() -> gitcp::result::Result<()> {
    let opt = gitcp::opt::Opt::from_args();

    let source = gitcp::source::Source::parse(opt.source)?;
    let bytes = download(source.download_url).await?;
    let tempdir = create_temporary_directory()?;
    unpack(bytes.as_ref(), tempdir.path())?;

    let path_buf = tempdir.path().join(source.glob_pattern);
    let source_glob_pattern = path_buf.to_str().unwrap();
    move_items(source_glob_pattern, &opt.destination)?;

    Ok(())
}

fn create_temporary_directory() -> gitcp::result::Result<tempfile::TempDir> {
    let value = tempfile::Builder::new().prefix("gitcp").tempdir()?;
    Ok(value)
}

async fn download(url: impl Into<String>) -> gitcp::result::Result<bytes::Bytes> {
    let client = reqwest::ClientBuilder::new().user_agent("gitcp").build()?;
    let response = client.get(url.into()).send().await?;
    if !response.status().is_success() {
        return Err(gitcp::error::Error::ResponseStatusError(response.status()));
    }
    let result = response.bytes().await?;
    Ok(result)
}

fn move_items(source_glob_pattern: &str, destination: &str) -> gitcp::result::Result<()> {
    let paths = list_moved_item_paths(source_glob_pattern);
    mkdir_p(destination)?;
    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.overwrite = true;
    fs_extra::move_items(&paths, destination, &copy_options)?;
    Ok(())
}

fn mkdir_p(path: &str) -> std::io::Result<()> {
    if let Err(e) = std::fs::create_dir_all(path) {
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            return Err(e);
        }
    }
    Ok(())
}

fn list_item_paths(glob_pattern: &str) -> Vec<std::path::PathBuf> {
    globwalk::glob(glob_pattern)
        .unwrap()
        .map(|path| path.unwrap().path().to_owned())
        .collect()
}

fn list_moved_item_paths(glob_pattern: &str) -> Vec<std::path::PathBuf> {
    let paths = list_item_paths(glob_pattern);
    if let Some(ignore_file) = gitcp::ignore_file::IgnoreFile::find(paths.clone().into_iter()) {
        let glob_set = ignore_file.to_glob_set();
        paths
            .iter()
            .filter(|path| !glob_set.is_match(path))
            .cloned()
            .collect()
    } else {
        paths
    }
}

fn unpack(
    readable: impl std::io::Read,
    destination: &std::path::Path,
) -> gitcp::result::Result<()> {
    let gz_encoder = flate2::read::GzDecoder::new(readable);
    let mut archive = tar::Archive::new(gz_encoder);
    archive.unpack(destination)?;
    Ok(())
}
