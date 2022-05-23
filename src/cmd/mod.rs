use std::{io, path, process, str::FromStr};

#[derive(Clone)]
struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

impl FromStr for Version {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r".*version ([\d]+)\.([\d]+)\.([\d]+).*").unwrap();
        let caps = re.captures(s).ok_or("Failed to match version")?;

        Ok(Version {
            major: caps[1].parse().map_err(|_| "Invalid major version")?,
            minor: caps[1].parse().map_err(|_| "Invalid minor version")?,
            patch: caps[1].parse().map_err(|_| "Invalid patch level")?,
        })
    }
}

pub struct Runner {
    cmd: path::PathBuf,
    version: Option<Version>,
}

impl Runner {
    pub fn new<P>(path: P) -> Runner
    where
        P: AsRef<path::Path>,
    {
        let cmd = path::PathBuf::from(path.as_ref());
        Runner { cmd, version: None }
    }

    fn eval_status(status: process::ExitStatus) -> Result<(), io::Error> {
        match status.code() {
            Some(code) if code == 0 => (),
            Some(code) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Process terminated with code {}", code),
                ));
            }
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::Interrupted,
                    "Process terminated by signal",
                ))
            }
        };
        Ok(())
    }

    pub fn get_version(&self) -> Option<String> {
        self.version
            .as_ref()
            .map(|v| format!("{}.{}.{}", v.major, v.minor, v.patch))
    }

    pub fn get_path(&self) -> path::PathBuf {
        self.cmd.clone()
    }

    pub fn validate(&mut self) -> Result<(), io::Error> {
        let cmd = process::Command::new(self.cmd.as_path())
            .arg("--version")
            .output()?;

        if let Err(err) = Runner::eval_status(cmd.status) {
            log::error!(
                "Execution failed:\n{}",
                String::from_utf8_lossy(&cmd.stderr)
            );
            return Err(err);
        }

        // example output of clang-format:
        // clang-format version 4.0.0 (tags/checker/checker-279)
        let stdout = String::from_utf8_lossy(&cmd.stdout);

        self.version = Some(stdout.parse::<Version>().map_err(|err| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to parse --version output {}: {}", stdout, err),
            )
        })?);
        Ok(())
    }

    fn run(mut cmd: process::Command) -> Result<(), io::Error> {
        let output = cmd.output()?;

        if let Err(err) = Runner::eval_status(output.status) {
            let stderr = String::from_utf8_lossy(&output.stderr);

            if stderr.len() != 0 {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("{}\n---\n{}---", err, stderr),
                ));
            }
            return Err(err);
        }
        Ok(())
    }

    pub fn run_tidy<P, Q, R>(
        &self,
        file: P,
        config_file: &Option<Q>,
        build_root: &Option<R>,
        fix: bool,
    ) -> Result<(), io::Error>
    where
        P: AsRef<path::Path>,
        Q: AsRef<path::Path>,
        R: AsRef<path::Path>,
    {
        let mut cmd = process::Command::new(self.cmd.as_path());

        cmd.arg(file.as_ref().as_os_str());
        if let Some(config_file) = config_file {
            cmd.arg(format!(
                "-config-file={}",
                config_file.as_ref().to_string_lossy()
            ));
        }
        if let Some(build_root) = build_root {
            cmd.arg(format!("-p={}", build_root.as_ref().to_string_lossy()));
        }
        if fix {
            cmd.arg("--fix");
        }
        cmd.arg("--quiet");

        Runner::run(cmd)
    }
}

impl Clone for Runner {
    fn clone(&self) -> Runner {
        Runner {
            cmd: path::PathBuf::from(self.cmd.as_path()),
            version: self.version.clone(),
        }
    }
}
