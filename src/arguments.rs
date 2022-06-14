use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(short, long)]
    pub searchtype: crate::parser::SearchType,
    #[clap(short, long)]
    pub name: String,
    #[clap(short, long)]
    pub folder: String,
    #[clap(short, long)]
    pub casesensitive: bool,
}
