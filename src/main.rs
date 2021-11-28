use sha2::{Digest, Sha256};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "hashlink")]
struct Opt {
    #[structopt(short, long)]
    secret: String,

    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn hash(key: String, path: PathBuf) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key);
    for component in path.components().rev() {
        match component {
            std::path::Component::Normal(p) => {
                if let Some(s) = p.to_str() {
                    hasher.update(s.as_bytes())
                }
            }
            _ => break,
        }
    }
    let result = hasher.finalize();
    return match path.extension() {
        Some(ext) => format!(
            "{}_{:x}.{}",
            path.file_stem()
                .and_then(|n| n.to_str())
                .expect("file_name unwrap failed"),
            result,
            ext.to_str().expect("ext unwrap failed"),
        ),
        None => format!(
            "{}_{:x}",
            path.file_name()
                .and_then(|n| n.to_str())
                .expect("file_name unwrap failed"),
            result,
        ),
    };
}

fn main() -> Result<(), ()> {
    let opt = Opt::from_args();

    println!("{}", hash(String::from("abcdefg"), opt.path));
    Ok(())
}
