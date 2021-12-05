use std::env;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "hashlink")]
struct Opt {
    #[structopt()]
    path: String,
}

fn main() -> Result<(), ()> {
    let opt = Opt::from_args();

    let secret: String = env::var("HASHLINK_SECRET_KEY").unwrap_or(String::new());
    println!("{}", hashlink::encrypt::encrypt(secret, opt.path));
    Ok(())
}
