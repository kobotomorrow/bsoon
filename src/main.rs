fn main() {
    if let Err(e) = bsoon::get_args().and_then(bsoon::run) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
