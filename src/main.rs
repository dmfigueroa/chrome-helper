use std::process::Command;

fn get_color_scheme() -> String {
    let mut gsettings = Command::new("color-schema");

    let output = gsettings.output().expect("Failed to get color scheme");
    let color_scheme =
        String::from_utf8(output.stdout).expect("Failed to convert color scheme to string");
    color_scheme.trim().replace("\"", "").to_string()
}

fn open_chrome() {
    let mut args = vec!["--restore-last-session"];
    // Print color scheme
    let color_scheme = get_color_scheme();
    if color_scheme.eq("dark") {
        args.push("--force-dark-mode");
    }
    Command::new("google-chrome-stable").args(args).spawn().expect("Failed to open Chrome");
}

fn main() {
    open_chrome();
}
