use gitcp::opt::Opt;

// For `from_args`.
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    let url = generate_download_url(&opt.source);
    let bytes = download(&url).await?;
    let tempdir = tempfile::Builder::new().tempdir()?;
    unpack(bytes.as_ref(), tempdir.path())?;

    let path_buf = tempdir.path().join("*/*");
    let source_glob_pattern = path_buf.to_str().unwrap();
    move_files(source_glob_pattern, &opt.destination)?;

    Ok(())
}

fn generate_download_url(source: &str) -> String {
    gitcp::source::Source::new(source.to_string()).download_url()
}

async fn download(url: &str) -> Result<bytes::Bytes, reqwest::Error> {
    let client = reqwest::ClientBuilder::new().user_agent("gitcp").build()?;
    client.get(url).send().await?.bytes().await
}

fn move_files(source_glob_pattern: &str, destination: &str) -> Result<u64, fs_extra::error::Error> {
    let mut path_bufs = Vec::new();
    glob::glob(source_glob_pattern).unwrap().for_each(|path| {
        let path = path.unwrap();
        path_bufs.push(path);
    });
    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.overwrite = true;
    fs_extra::move_items(&path_bufs, destination, &copy_options)
}

fn unpack(
    readable: impl std::io::Read,
    destination: &std::path::Path,
) -> Result<(), std::io::Error> {
    let gz_encoder = flate2::read::GzDecoder::new(readable);
    let mut archive = tar::Archive::new(gz_encoder);
    archive.unpack(destination)
}
