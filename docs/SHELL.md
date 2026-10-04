# NexOS kernel shell

The current kernel shell is a small bring-up/debugging shell, not a POSIX shell or a Windows command prompt. It runs inside the kernel and is intended for QEMU and development hardware.

## Commands

| Command | Behavior |
| --- | --- |
| `help` | Show supported commands |
| `clear` | Clear the VGA text screen |
| `echo TEXT` | Print text |
| `uname` | Print the OS name and architecture |
| `version` | Print the development version status |
| `status` | Show current experimental subsystem status |
| `pwd` | Print the current root path (`/`) |
| `ls` | Show the known top-level VFS directories |
| `cat /path` | Read a file from the in-memory VFS |
| `touch /path` | Create an empty file |
| `mkdir /path` | Create a directory entry |
| `write /path TEXT` | Create or overwrite a file with text |
| `net`, `wifi`, `hotspot`, `bluetooth` | Report networking implementation status |
| `reboot` | Request a legacy keyboard-controller reset |

## Example

```text
nexos> cat /etc/motd
Welcome to NexOS.
nexos> write /tmp/hello.txt Hello from NexOS
nexos> cat /tmp/hello.txt
Hello from NexOS
nexos> touch /home/notes.txt
nexos> mkdir /home/demo
```

Use absolute paths beginning with `/`. The VFS is a fixed-capacity in-memory prototype: it has a maximum of 64 entries in this boot path, file contents are limited to 4096 bytes per file, and all changes are lost when the machine reboots. The current `ls` command prints the known root namespace rather than enumerating user-created files. Directory traversal, parent-directory validation, permissions, persistent storage, pipes, redirection, quoting, and command history are not implemented.

The `write` command treats everything after the first space following the path as file contents. It is a small diagnostic helper, not a full shell parser.

## Hardware and safety

Test in QEMU first. This shell is not a replacement for a stable recovery console, and NexOS is not ready for daily-driver use. Wi-Fi, hotspot, Bluetooth, and usable network connectivity are not provided by these commands.
