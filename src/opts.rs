use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "indexer", about = "Verge Indexer")]
pub struct Opts {
    #[structopt(long = "wipe-whole-db")]
    pub wipe_db: bool,
}
