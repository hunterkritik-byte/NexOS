use core::fmt::Write;
use super::tty::Terminal;

pub struct Shell {
    cwd: &'static str,
}

impl Shell {
    pub const fn new() -> Self { Self { cwd: "/" } }

    pub fn prompt<W: Write>(&self, terminal: &mut Terminal<W>) {
        let _ = write!(terminal, "nexos:{}$ ", self.cwd);
    }

    pub fn execute<W: Write>(&mut self, terminal: &mut Terminal<W>, command: &str) {
        let command = command.trim();
        if command.is_empty() { return; }

        let mut parts = command.split_ascii_whitespace();
        match parts.next() {
            Some("help") => {
                terminal.write_str("help  clear  pwd  ls  cd  echo  uname  ps  reboot\n");
            }
            Some("clear") => {
                terminal.write_str("\x1b[2J\x1b[H");
            }
            Some("pwd") => {
                let _ = writeln!(terminal, "{}", self.cwd);
            }
            Some("ls") => {
                terminal.write_str("bin  dev  etc  home  proc  sbin  tmp  var\n");
            }
            Some("cd") => {
                let path = parts.next().unwrap_or("/");
                if path == "/" {
                    self.cwd = "/";
                } else {
                    terminal.write_str("cd: directory exists in VFS namespace only; persistent VFS navigation is next\n");
                }
            }
            Some("echo") => {
                let mut first = true;
                for part in parts {
                    if !first { terminal.write_str(" "); }
                    terminal.write_str(part);
                    first = false;
                }
                terminal.write_str("\n");
            }
            Some("uname") => {
                terminal.write_str("NexOS x86_64\n");
            }
            Some("ps") => {
                terminal.write_str("PID  STATE   NAME\n1    running init\n");
            }
            Some("reboot") => {
                terminal.write_str("reboot: ACPI reset support is not installed yet\n");
            }
            Some(other) => {
                let _ = writeln!(terminal, "{other}: command not found");
            }
            None => {}
        }
    }
}
