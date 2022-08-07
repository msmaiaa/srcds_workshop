use arboard::Clipboard;
use clap::Parser;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    path: String,
}

fn get_files(path: &str) -> Vec<String> {
    let mut files = Vec::new();
    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            files.push(entry.path().to_str().unwrap().to_string());
        }
    }
    files
}

fn filter_files(files: &Vec<String>) -> Vec<String> {
    files
        .into_iter()
        .filter_map(|file| {
            let file = file.to_string();
            if file.ends_with(".bsp") {
                Some(file)
            } else {
                None
            }
        })
        .map(|file| {
            file.replace("\\", "/")
                .replace(".bsp", "")
                .split("maps")
                .nth(1)
                .unwrap()
                .to_string()
        })
        .map(|file| {
            let mut f = file.chars();
            f.next();
            f.as_str().to_string()
        })
        .collect()
}

fn main() {
    let mut clipboard = Clipboard::new().unwrap();
    let args = Args::parse();
    let files = get_files(&args.path);
    let res = filter_files(&files).join("\n");
    println!("{}", res);
    clipboard.set_text(res).unwrap();
}
