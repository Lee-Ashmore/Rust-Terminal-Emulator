Author: Lee Ashmore
Email: JLeeashmore@gmail.com

This is a basic terminal emulator created in Rust. 

Goals for this project:
	1. Create a functional terminal emulator on mac
	2. Use rust as the Primary language


Useful Resources:
	~https://vt100.net/emu/dec_ansi_parser
		A description of a VT500 terminal and the logic associated

	~run "man console" or "man tty" in terminal
		Documentation on console and tty 

	~https://www.win.tue.nl/~aeb/linux/lk/lk-10.html
		Information on process creation and control

	~https://www.uninformativ.de/blog/postings/2018-02-24/0/POSTING-en.html
		A very basic terminal emulator written in C

	~https://github.com/psinghal20/rush/blob/master/src/main.rs
		A simple rust implementation. 


Branching
	A git workflow branch scheme is implemented. All branches should be created
	off of the dev branch. When creating branches they should be labeled with a
	flag indicating the type of branch.	The current branch types are 
		-R: Research branch. A branch for experimentation and research for a 
		given idea or feature.
		-F: Feature branch. A branch for the implementation of a new feature.
		
