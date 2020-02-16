use std::fs::File;
use std::os::unix::io::FromRawFd;
use std::process::{Child, Command, Stdio};

use nix::pty::openpty;

// Kill the program. For use if something goes wrong
macro_rules! kill {
    ($($arg:tt)*) => {
        std::process::exit(1);
    };
}

pub struct Shell {
    program: String,
}

impl Shell {
    pub fn new(program: &str) -> Shell {
        let shell = Shell {
            program: String::from(program),
        };
        shell
    }
}

pub struct Pty {
    child: Child,
    file_descriptor: File,
}

pub fn new() -> Pty {
    // If there is a window for the process, then I need to pass in window size here
    let fd = openpty(None, None).expect("Failed to open Pty");

    let master = fd.master;
    let slave = fd.slave;

    // TODO: Once user configs are added, put this behind a if/else
    let default_shell = Shell::new(&"/bin/bash");

    //TODO: Add config/args for shell
    let mut shell = Command::new(default_shell.program);
    shell.stdin(unsafe { Stdio::from_raw_fd(slave) });
    shell.stdout(unsafe { Stdio::from_raw_fd(slave) });
    shell.stderr(unsafe { Stdio::from_raw_fd(slave) });

    match shell.spawn() {
        Ok(process) => {
            let pty = Pty {
                child: process,
                file_descriptor: unsafe { File::from_raw_fd(master) },
            };

            return pty;
        }
        Err(err) => {
            println!("The proccess could not be spawned: {}", err);
            kill!();
        }
    };
}
