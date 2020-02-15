use std::fs::File;
use std::process::Child;
use nix::pty::openpty

pub struct Pty {
    child: Child,
    file_descriptor: File,
}

pub fn new(winsize) -> Pty {
    // create the file descriptors for master and slave
        // the pty needs a window size to be created...
    fd_ends = openpty(Some(winsize), None);
    (master, slave) = fd_ends.master, fd_ends.slave;

    // set the shell to comunicate with the slave side of the pty
}

// I want a class that will track all the input and output from a given process
// The only process that this has to run, is the shell. Let the shell handle
// everything else.
