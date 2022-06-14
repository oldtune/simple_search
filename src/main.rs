mod arguments;
mod parser;
use clap::Parser;
mod finder;
use arguments::Args;
use finder::Finder;

use crate::parser::SearchType;
mod error;

fn main() {
    let path = "/home/fedora/Downloads/Book";
    // let arg = arguments::Args::parse();
    let arg = Args {
        searchtype: SearchType::Fuzzy,
        name: "architecture".to_string(),
        folder: path.to_owned(),
        casesensitive: true,
    };

    let finder = Finder {
        file_name: arg.name,
        file_path: arg.folder,
    };

    let file_names = finder.find();

    for file_name in file_names.unwrap() {
        println!("{}", file_name);
    }

    // let files = get_files(path);
    // if fuzzy {
    //     print_out(fuzzy_search(files));
    //     return;
    // }
    // if normal_search {
    //     print_out(contain_search(files));
    // }

    // print(search_using_regex(files));
}
