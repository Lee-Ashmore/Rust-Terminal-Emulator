use terminal::tty;

fn main() {
    println!("Welcome to rust terminal emulator");
    // Perform Setup
    // TODO:

    // Create Display
    // TODO:

    // run terminal
    // TODO:
    run();
}

fn run() {
    // create terminal process
    // TODO:

    // create Pty [in progress]
    let pty = tty::new();
    println!("{:?}", pty);
    // create Pty I/O loop
    // TODO:

    // create event handler loop
    // TODO:

    // kickoff event handler as primary runtime
    // TODO:
    println!("run");
}
