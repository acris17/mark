pub mod args;
pub mod data;

use xdg::BaseDirectories;

use crate::cmd;
use args::{Args, Commands};
use data::AppData;

pub struct App {
    args: Args,
    app_data: AppData,
    base_dirs: BaseDirectories,
}
impl App {
    pub fn new(args: Args, app_data: AppData, base_dirs: BaseDirectories) -> App {
        App {
            args,
            app_data,
            base_dirs,
        }
    }
    pub fn run(&self) {
        let command = &self.args.command;
        let marks = &self.app_data.marks;
        let base_dirs = &self.base_dirs;

        match command {
            Commands::Open { name } => {
                cmd::open(marks, name);
            }
            Commands::Edit => {
                cmd::edit(base_dirs);
            }
            Commands::Get { name } => {
                cmd::get(marks, name);
            }
        }
    }
}
