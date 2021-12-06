use std::env;
use std::os::unix::fs;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "hashlink")]
struct Opt {
    #[structopt()]
    path: String,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    let secret: String = env::var("HASHLINK_SECRET_KEY").unwrap_or(String::new());
    let link = hashlink::encrypt::encrypt(secret, opt.path.clone());

    let mut original = env::current_dir().unwrap();
    original.push(opt.path);

    let mut dist = env::current_dir().unwrap();
    dist.push(env::var("HASHLINK_DIST").unwrap_or("dist".to_string()));
    dist.push(link);

    fs::symlink(original, dist)
}
