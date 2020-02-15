Goals for this project:
	1. Meet VT100 specifications
	2. Use rust as the Primary language
	
Stretch Goals:
	1. Meet ECMA-35 specification 
	2. Meet ECMA-43 specification
	3. Meet ECMA-48 specification

Resources
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

Flow:
	Create session for precess and child process
	Runtime loop:
		Take in input
		Parse input
			Parse ANSI (later on unicode 8)
		Dispatch input to child processes (shell, etc)
		Recieve output from child
		Display output

Requirerments:
	~Visual Interface
	~Command parser
	~Stdin and Stdout interfaces

Architecture:

Branching
	All branches should be labeled with a flag indicating the type of branch.
	the current branch types are 
		-R: Research branch. A branch for experimentation and research for a 
		given idea or feature.
		-F: Feature branch. A branch for the implementation of a new feature.
		
