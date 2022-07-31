// For `Opt::from_args`.
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = gitcp::opt::Opt::from_args();

    let source = gitcp::source::Source::new(opt.source);
    let url = source.download_url().unwrap();
    let bytes = download(&url).await?;
    let tempdir = tempfile::Builder::new().tempdir()?;
    unpack(bytes.as_ref(), tempdir.path())?;

    let path_buf = tempdir.path().join(source.glob_pattern());
    let source_glob_pattern = path_buf.to_str().unwrap();
    move_items(source_glob_pattern, &opt.destination)?;

    Ok(())
}

async fn download(url: &str) -> Result<bytes::Bytes, reqwest::Error> {
    let client = reqwest::ClientBuilder::new().user_agent("gitcp").build()?;
    client.get(url).send().await?.bytes().await
}

fn move_items(source_glob_pattern: &str, destination: &str) -> Result<u64, fs_extra::error::Error> {
    let paths = list_moved_item_paths(source_glob_pattern);
    mkdir_p(destination)?;
    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.overwrite = true;
    fs_extra::move_items(&paths, destination, &copy_options)
}

fn mkdir_p(path: &str) -> std::io::Result<()> {
    if let Err(e) = std::fs::create_dir_all(path) {
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            return Err(e);
        }
    }
    Ok(())
}

fn list_moved_item_paths(glob_pattern: &str) -> Vec<std::path::PathBuf> {
    globwalk::glob(glob_pattern)
        .unwrap()
        .map(|path| path.unwrap().path().to_owned())
        .collect()
}

fn unpack(
    readable: impl std::io::Read,
    destination: &std::path::Path,
) -> Result<(), std::io::Error> {
    let gz_encoder = flate2::read::GzDecoder::new(readable);
    let mut archive = tar::Archive::new(gz_encoder);
    archive.unpack(destination)
}
