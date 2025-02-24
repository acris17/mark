use std::process::Command;

pub fn shell(process: &str) {
    let Ok(mut child) = Command::new("sh").arg("-c").arg(process).spawn() else {
        return;
    };

    let _ = child.wait();
}
