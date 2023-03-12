use directories::{BaseDirs, ProjectDirs, UserDirs};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use std::{fmt, fs};

use std::process::{Command, Stdio, Child, ChildStdout};

#[derive(Default, Debug)]
struct Metric {
    timestamp: u128,
    energy_now: i32,
    capacity: i8,
}

impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(T:{}, C:{}, EN:{})",
            self.timestamp, self.capacity, self.energy_now
        )
    }
}

fn timestamp() -> u128 {
    use std::time::SystemTime;
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() // See struct std::time::Duration methods
}

fn grep_cmd(ipt: ChildStdout) -> Child {
    let gre_outut_child = Command::new("grep")
        .arg("-v")
        .arg("top")
        .stdin(ipt)
        .stdout(Stdio::piped())
        .spawn()
        .expect("grep failed");

    return gre_outut_child;

}

fn active_high_usage_processes() -> Result<(), Box<dyn Error>> {
    // top -b -n 1 | grep -v 'top' | head -n 12  | tail -n 5 | awk '{print $10" "$12}'
    let mut top_output_child = Command::new("top")
        .arg("-b")
        .arg("-n")
        .arg("1")
        .arg("-o")
        .arg("%CPU")
        // .arg("Hello world")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    if let Some(top_output) = top_output_child.stdout.take() {

        let mut grep_output_child = grep_cmd(top_output);

        top_output_child.wait()?;
        if let Some(grep_output) = grep_output_child.stdout.take() {

            let mut head_output_child = Command::new("head")
                .arg("-n")
                .arg("12")
                .stdin(grep_output)
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed head command");

            grep_output_child.wait()?;

            if let Some(head_output) = head_output_child.stdout.take() {
                let mut tail_output_child = Command::new("tail")
                    .arg("-n")
                    .arg("5")
                    .stdin(head_output)
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("Failed tail command");

                // let tail_stdout = tail_output_child.wait_with_output()?;

                head_output_child.wait()?;
                tail_output_child.wait()?;

                // println!("aa: {}", String::from_utf8(tail_stdout.stdout).unwrap());

                if let Some(tail_output) = tail_output_child.stdout.take() {
                    let awk_output_child = Command::new("awk")
                        .arg("{print $9\" \"$12}")
                        .stdin(tail_output)
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("Failed awk command");

                    let awk_stdout = awk_output_child.wait_with_output()?;

                    head_output_child.wait()?;

                    println!("{}", String::from_utf8(awk_stdout.stdout).unwrap());
                }
            }

        }

    }

    // println!("{:?}", output);
    Ok(())
}

fn save_record(metric: Metric) -> Result<(), Box<dyn Error>> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "konsv", "batterymeter") {
        let path = proj_dirs.config_dir();
        let str_path = Box::new(path.to_str().unwrap());

        let ppath = Path::new(*str_path);
        if Path::exists(ppath) {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(*str_path)?;

            if let Err(e) = writeln!(file, "{}", metric) {
                eprintln!("Couldn't write to file: {}", e);
            }
            // fs::write(path, metric.to_string().as_bytes()).unwrap();
        } else {
            // File::create(path).unwrap();
            fs::write(*str_path, format!("{}\n", metric).as_bytes()).unwrap();
        }
        return Ok(());
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // f = fs:
    let mut metric: Metric = Metric::default();
    let mut buf = String::new();
    {
        let mut f = File::open("/sys/class/power_supply/BAT1/energy_now")?;
        f.read_to_string(&mut buf)?;

        metric.energy_now = buf.trim_end().parse().unwrap();
    }

    buf.clear();
    {
        let mut f = File::open("/sys/class/power_supply/BAT1/capacity")?;
        f.read_to_string(&mut buf)?;

        metric.capacity = buf.trim_end().parse().unwrap();
    }

    metric.timestamp = timestamp();

    // let capacity: i8 = buf.parse()?;

    // println!("{}", energy_now);
    println!("{}", metric);
    active_high_usage_processes().unwrap();
    save_record(metric)?;
    Ok(())
}
