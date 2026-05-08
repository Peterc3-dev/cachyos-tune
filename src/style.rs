/// Phosphor-green ANSI color codes.
const GREEN: &str = "\x1b[38;2;0;255;200m";
const DIM_GREEN: &str = "\x1b[38;2;0;128;100m";
const BRIGHT_GREEN: &str = "\x1b[38;2;160;255;230m";
const YELLOW: &str = "\x1b[38;2;200;200;40m";
const RED: &str = "\x1b[38;2;255;80;60m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

pub fn header(text: &str) {
    println!(
        "\n{}{}  {} {}",
        BOLD, GREEN, text, RESET
    );
    println!(
        "{}{}{}",
        DIM_GREEN,
        "─".repeat(text.len() + 4),
        RESET
    );
}

pub fn print_applied(knob: &str, value: &str, note: &str) {
    if note.is_empty() {
        println!(
            "  {} {} {}{}{} {}",
            GREEN, ">>", BRIGHT_GREEN, knob, RESET,
            format!("{}= {}{}", GREEN, value, RESET),
        );
    } else {
        println!(
            "  {} {} {}{}{} {} {}({})",
            GREEN, ">>", BRIGHT_GREEN, knob, RESET,
            format!("{}= {}{}", GREEN, value, RESET),
            DIM_GREEN, note,
        );
    }
}

pub fn print_warn(msg: &str) {
    eprintln!("  {}!! {}{}", YELLOW, msg, RESET);
}

pub fn print_error(msg: &str) {
    eprintln!("  {}!! {}{}", RED, msg, RESET);
}

pub fn green(text: &str) -> String {
    format!("{}{}{}", GREEN, text, RESET)
}

pub fn bright_green(text: &str) -> String {
    format!("{}{}{}", BRIGHT_GREEN, text, RESET)
}

pub fn dim_green(text: &str) -> String {
    format!("{}{}{}", DIM_GREEN, text, RESET)
}

pub fn yellow(text: &str) -> String {
    format!("{}{}{}", YELLOW, text, RESET)
}

pub fn bold_green(text: &str) -> String {
    format!("{}{}{}{}", BOLD, GREEN, text, RESET)
}

pub fn banner() {
    println!(
        "{}{}",
        GREEN,
        r#"
   ___           _           ___  ___   _____
  / __\__ _  ___| |__  _   _/___\/ __\ /__   \_   _ _ __   ___
 / /  / _` |/ __| '_ \| | | //  / /      / /\/ | | | '_ \ / _ \
/ /__| (_| | (__| | | | |_| / \_/ /___   / /  | |_| | | | |  __/
\____/\__,_|\___|_| |_|\__, \___/\____/  \/    \__,_|_| |_|\___|
                        |___/
"#
    );
    println!(
        "  {}System tuner for CachyOS on AMD APU{}",
        DIM_GREEN, RESET
    );
}
