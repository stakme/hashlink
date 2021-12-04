use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "hashlink")]
struct Opt {
    #[structopt(short, long)]
    secret: String,

    #[structopt()]
    path: String,
}

fn main() -> Result<(), ()> {
    let opt = Opt::from_args();

    println!("{}", hashlink::encrypt::encrypt(opt.secret, opt.path));

    Ok(())
}
