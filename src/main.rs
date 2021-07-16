use chrono::{Datelike, Local};
use std::env;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    // Use $XDG_RUNTIME_DIR if present, else use /tmp
    let dir_name = match env::var("XDG_RUNTIME_DIR") {
        Ok(dir) => dir,
        _ => "/tmp".to_owned(),
    };
    let dir_path = Path::new(&dir_name);

    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(dir_path.join("calendar_notification_month"))?;

    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    let mut diff: i32 = contents.trim().parse().unwrap_or(0);

    diff = match env::args().nth(1).as_deref() {
        Some("next") => diff + 1,
        Some("prev") => diff - 1,
        Some("curr") => 0,
        _ => diff,
    };

    let output = Command::new("cal")
        .arg("--")
        .arg(format!("{:+} months", diff))
        .output()?;

    let mut cal_out = String::from_utf8(output.stdout)?;

    // If current month, highlight the date
    if diff == 0 {
        let today = Local::now().day();
        cal_out = cal_out.replacen(
            format!(" {} ", today).as_str(),
            format!(" <u><b>{}</b></u> ", today).as_str(),
            1,
        );
    }

    // Retrieve the position of the first linebreak
    let first_line_end = (|| {
        for (i, c) in cal_out.chars().enumerate() {
            if c == '\n' {
                return i;
            }
        }
        return 0;
    })();

    let title = &cal_out[..first_line_end];
    let body = cal_out[first_line_end + 1..].trim_end();

    // Display notification
    Command::new("dunstify")
        .args(&[
            "-h",
            "string:x-canonical-private-synchronous:calendar",
            title,
            format!("{}\n\n<i>       ~ calendar</i> 󰸗 ", body).as_ref(),
        ])
        .output()?;

    // Truncate and write to file
    f.set_len(0)?;
    f.seek(SeekFrom::Start(0))?;
    write!(f, "{}", diff)?;
    Ok(())
}
