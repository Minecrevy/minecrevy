use std::fs::DirEntry;
use std::iter::once;
use std::path::Path;
use std::process::Command;

fn prepend_name(entry: DirEntry) -> String {
    format!("minecrevy_{}", entry.file_name().to_str().unwrap())
}

fn is_dir(entry: &DirEntry) -> bool {
    matches!(entry.file_type().map(|ty| ty.is_dir()), Ok(true))
}

fn crate_names(dir: &Path) -> impl Iterator<Item=String> {
    std::fs::read_dir(dir).unwrap()
        .flatten()
        .filter(is_dir)
        .map(prepend_name)
}

fn create_filter() -> impl Iterator<Item=String> {
    let api = crate_names(Path::new("api"));
    let server = crate_names(Path::new("server"));
    let main = "minecrevy".to_owned();

    api.chain(server).chain(once(main))
}

fn main() {
    // transitive deps
    Command::new("cargo")
        .args(["deps", "-o", "dep-graph.dot", "--filter"])
        .args(create_filter())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    Command::new("dot")
        .args(["-Tpng", "-O", "dep-graph.dot"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // no transitive deps
    Command::new("cargo")
        .args(["deps", "-o", "dep-graph-simple.dot", "--no-transitive-deps", "--filter"])
        .args(create_filter())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    Command::new("dot")
        .args(["-Tpng", "-O", "dep-graph-simple.dot"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
