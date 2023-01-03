use std::process;
use std::str::from_utf8;
use crate::client::ADB_PROGRAM;
use crate::structs::App;

pub fn get_installed_apps(dev_id: &str) ->  Option<Vec<App>> {
    if let Ok(output) = process::Command::new(ADB_PROGRAM).args([
        "-s", dev_id, "shell", "dumpsys", "package", "packages"
    ]).output() {
        if output.status.success() {
            if let Ok(output) = from_utf8(&output.stdout) {
                let mut builder = App::builder();
                let mut out = vec![];

                let output = output.split("\n");
                for line in output {
                    let line = line.trim();
                    if line.starts_with("Package [") {
                        if let Some(app) = builder.build() {
                            out.push(app);
                        }
                        builder = App::builder();
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() != 3 {
                            continue
                        }

                        builder.id(parts.get(1).unwrap().trim().replace("[", "").replace("]", ""));
                        continue;
                    }

                    if line.starts_with("versionName=") {
                        let parts: Vec<&str> = line.split("=").collect();
                        if parts.len() < 2 {
                            continue;
                        }
                        builder.version(String::from(parts.get(1).unwrap().trim()));
                        continue;
                    }

                    if line.starts_with("flags=") {
                        let parts: Vec<&str> = line.split("=").collect();
                        if parts.len() != 2 {
                            continue;
                        }
                        let flags = parts.get(1).unwrap().trim().replace("[", "").replace("]", "");

                        let flags: Vec<&str> = flags.trim().split_whitespace().collect();
                        if flags.contains(&"SYSTEM") {
                            builder.system(true);
                        }
                    }
                }

                if let Some(app)  = builder.build() {
                    out.push(app);
                }

                return Some(out);
            }
        }
    }

    None
}