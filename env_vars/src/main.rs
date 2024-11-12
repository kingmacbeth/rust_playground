fn main() {
    let cpu_arch = std::env::consts::ARCH;
    let os = std::env::consts::OS;
    let family = std::env::consts::FAMILY;
    println!("\n CPU ARCHITECTURE: {cpu_arch}\n OS: {os}\n FAMILY: {family}");
}
