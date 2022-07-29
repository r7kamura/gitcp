use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Copy files from Git repository to local.")]
pub struct Opt {
    #[structopt(help = "GitHub repository name (e.g. r7kamura/gitcp)")]
    pub source: String,

    #[structopt(default_value = ".", help = "Path to destination directory")]
    pub destination: String,
}
