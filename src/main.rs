use std::process;

use clap::Parser;
use xdg::BaseDirectories;

use mark::app::{App, args::Args, data::AppData};

pub fn main() {
    let app_name = env!("CARGO_PKG_NAME");

    let base_dirs = match BaseDirectories::with_prefix(app_name) {
        Ok(base_dirs) => base_dirs,
        Err(e) => {
            eprintln!("Error: Could not get xdg base directories: {e}");
            process::exit(1);
        }
    };

    let app_data = match AppData::new(&base_dirs) {
        Ok(app_data) => app_data,
        Err(e) => {
            eprintln!("Error: Could not get app data: {e}");
            process::exit(1);
        }
    };

    let args = Args::parse();
    let app = App::new(args, app_data, base_dirs);

    app.run();
}
