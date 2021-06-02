# novavim
I had coded out a <a href="https://github.com/solidiquis/novavim_go">proof of concept in Go</a> a few months ago and figured this would be fun to tackle in Rust.

The goal of this project is mainly to learn and practice Rust and hopefully end up with something that is at the very least stable and usable. A secondary goal is to make this project as lightweight as possible and therefore do as many things from scratch as possible; this means I won't be relying on ncurses nor any terminal UI drawing libraries — these will be done in-house. The only two non-standard-lib crates that I envision myself using are the libc crate (to make ioctl system calls) and regex crate.

## Brief overview of how NovaVim works
NovaVim leverages the `stty` POSIX command to adjust the behavior of the Termios layer sitting between the master and slave pty so that user-input is unbuffered and unechoed which allows keypresses to be automatically available to the program without having to provide a new-line.

When the key arrives to NovaVim, the program will first try to determine the key that is pressed: Enter, Tab, directional-key, etc.. Once the key-type is ascertained, the NovaVim mux (`Mux` struct) will try to determine which controller (e.g. `InsertCtrl` struct) to send the key to, while keeping track of which is the active controller. Once the appropriate controller has been determined, the key will then be forwarded to the appropriate controller action (methods implemented on controller) to be handled.

Cursor movement, text-editing, and terminal ui behaviors are all driven by `blurses`, which is my janky in-house version of ncurses. All of the text that is echoed back to the user is cached and edit-history is also tracked.

Example flow A:

1. <i> -> <Mux mode: Normal> -> <Mux mode: Insert>
2. <h> -> <Mux mode: Insert> -> <InsertCtrl> -> Forward to `handle_regular_key` action -> echo character back to user.
