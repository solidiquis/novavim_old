use std::process::Command;

pub fn unbuffer_stdin() {
    Command::new("stty")
            .arg("-f")
            .arg("/dev/tty")
            .arg("cbreak")
            .arg("min")
            .arg("1")
            .output()
            .expect("Failed to unbuffer stdin.");

    ()
}

pub fn unecho_stdin() {
    Command::new("stty")
            .arg("-f")
            .arg("/dev/tty")
            .arg("-echo")
            .output()
            .expect("Failed to unecho stdin.");

    ()
}
