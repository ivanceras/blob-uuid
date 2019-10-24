use blob_uuid::Uuid;
use std::io;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Default)]
#[structopt(name = "blob-uuid")]
pub struct Opt {
    #[structopt(
        help = "Convert the input uuid to blob string, if there is no argument a random uuid will be used"
    )]
    input: Option<String>,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    if let Some(input) = opt.input {
        let uuid = Uuid::parse_str(&input).expect("In valid uuid");
        let blob = blob_uuid::to_blob(&uuid);
        println!("{}", blob);
    } else {
        let blob = blob_uuid::random_blob();
        println!("{}", blob);
    }
    Ok(())
}
