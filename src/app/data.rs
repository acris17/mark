use std::error::Error;
use std::fs::{self, File};

use toml::Table;
use xdg::BaseDirectories;

pub struct AppData {
    pub marks: Table,
}
impl AppData {
    pub fn new(base_dirs: &BaseDirectories) -> Result<AppData, Box<dyn Error>> {
        let mark_file = base_dirs.place_data_file("marks.toml")?;

        if !mark_file.exists() {
            File::create(&mark_file)?;
        }

        let contents = fs::read_to_string(mark_file)?;
        let marks = contents.parse::<Table>()?;
        let app_data = AppData { marks };

        Ok(app_data)
    }
}
