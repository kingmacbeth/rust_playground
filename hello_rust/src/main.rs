fn main() {
    let read_files = |path: &str| std::fs::read_to_string(path);

    println!("Path: {:?}", read_files());
}
