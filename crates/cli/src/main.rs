use cli::app;

fn main() {
    if let Err(err) = app::run() {
        eprintln!("{err}");
    }
}
