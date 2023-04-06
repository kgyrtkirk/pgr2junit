
use std::path::Path;

use clap::Parser;
use pgr2junit::pgr_model::Model;
mod pgr_model;


/// pg_regress to junit transformer
#[derive(Parser)]
struct Cli {
    /// should be passed to write the out xml
    #[clap(long, short)]
    write: bool,
    /// the path the tool supposed to process
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("parsing data from: {:?}",args.path);
    let model = Model::new(&args.path);
    if args.write {
        println!("writing out: {:?}",args.path);
        model.save();
    }

}
