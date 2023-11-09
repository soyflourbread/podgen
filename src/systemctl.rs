use std::process::{Command, ExitStatus, Stdio};

pub struct SystemCtl {
    exec_path: String,

    user: bool, // User units: https://wiki.archlinux.org/title/systemd/User
}

const SYSTEMCTL_DEFAULT_PATH: &str = "/usr/bin/systemctl";

impl SystemCtl {
    pub fn new() -> Self {
        Self {
            exec_path: SYSTEMCTL_DEFAULT_PATH.into(),
            user: false,
        }
    }

    pub fn new_user() -> Self {
        Self {
            exec_path: SYSTEMCTL_DEFAULT_PATH.into(),
            user: true,
        }
    }
}

impl SystemCtl {
    pub fn daemon_reload(&self) -> std::io::Result<ExitStatus> {
        self.to_cmd().arg("daemon-reload").spawn()?.wait()
    }
}

impl SystemCtl {
    fn to_cmd(&self) -> Command {
        let mut ret = Command::new(self.exec_path.clone());
        ret.stdout(Stdio::piped());
        ret.stderr(Stdio::null());

        if self.user {
            ret.arg("--user");
        }

        ret
    }
}
