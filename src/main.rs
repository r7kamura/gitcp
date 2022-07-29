use gitcp::opt::Opt;

// For `from_args`.
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    let bytes = download_tar_gz(&opt.source).await?;
    let tempdir = tempfile::Builder::new().tempdir()?;
    unpack_tar_gz(&bytes, tempdir.path())?;

    let path_buf = tempdir.path().join("*/*");
    let source_glob_pattern = path_buf.to_str().unwrap();
    move_files(source_glob_pattern, &opt.destination)?;

    Ok(())
}

fn generate_tar_gz_url(source: &str) -> String {
    format!("https://api.github.com/repos/{}/tarball/", source)
}

async fn download(url: &str) -> Result<bytes::Bytes, reqwest::Error> {
    let client = reqwest::ClientBuilder::new().user_agent("gitcp").build()?;
    client.get(url).send().await?.bytes().await
}

async fn download_tar_gz(source: &str) -> Result<bytes::Bytes, reqwest::Error> {
    let url = generate_tar_gz_url(source);
    download(&url).await
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

fn unpack_tar_gz(
    bytes: &bytes::Bytes,
    destination: &std::path::Path,
) -> Result<(), std::io::Error> {
    println!("{:?}", bytes);
    let gz_encoder = flate2::read::GzDecoder::new(bytes.as_ref());
    let mut archive = tar::Archive::new(gz_encoder);
    archive.unpack(destination)
}
