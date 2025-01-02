# Ratatui
{id: ratatui}

## What is Ratatui?
{id: what-is-ratatui}

* TUI = Terminal User Interface
* [Ratatui](https://ratatui.rs/)

## Ratatui Showcases
{id: rataturi-showcases}

From [Ratatui Apps](https://ratatui.rs/showcase/apps/)

* [atuin](https://github.com/atuinsh/atuin) - Atuin replaces your existing shell history with a SQLite database, and records additional context for your commands.
* [bandwhich](https://github.com/imsnif/bandwhich) - A CLI utility for displaying current network utilization by process, connection and remote IP/hostname.
* [bottom](https://github.com/ClementTsang/bottom) - A customizable cross-platform graphical process/system monitor for the terminal.
* [csvlens](https://github.com/YS-L/csvlens) - A command line CSV file viewer. It is like less but made for CSV.
* [dua](https://github.com/Byron/dua-cli) - a disk space analysis tool designed for speed, leveraging parallel processing to quickly provide detailed disk usage information and allowing for faster deletion of unnecessary data compared to the standard ‘rm’ command.
* [joshuto](https://github.com/kamiyaa/joshuto) - Ranger-like terminal file manager written in Rust.
* [fzf-make](https://github.com/kyu08/fzf-make) - A command line tool that executes make target using fuzzy finder with preview window.
* [gitui](https://github.com/extrawurst/gitui) - TUI for git written in rust.
* [gpg-tui](https://github.com/orhun/gpg-tui) - gpg-tui is a Terminal User Interface for GnuPG.
* [material](https://github.com/azorng/material) - A material design color palette for the terminal.
* [minesweep-rs](https://github.com/cpcloud/minesweep-rs) - A mine sweeping game written in Rust.
* [oatmeal](https://github.com/dustinblackman/oatmeal/) - Oatmeal is a terminal UI chat application that speaks with LLMs.
* [oha](https://github.com/hatoo/oha/) - oha sends some load to a web application and show realtime tui.
* [rucola](https://github.com/Linus-Mussmaecher/rucola) - An application to manage markdown notes from your terminal and compile them to HTML.
* [scope-tui](https://github.com/alemidev/scope-tui) - A simple oscilloscope/vectorscope/spectroscope for your terminal.
* [slumber](https://github.com/LucasPickering/slumber) - Terminal HTTP/REST client.
* [taskwarrior-tui](https://github.com/kdheepak/taskwarrior-tui) - A terminal user interface for taskwarrior.
* [xplr](https://github.com/sayanarijit/xplr) - A hackable, minimal, fast TUI file explorer.
* [yazi](https://yazi-rs.github.io/) - Blazing fast terminal file manager written in Rust, based on async I/O.
* [openapi-tui](https://github.com/zaghaghi/openapi-tui) - Unlock the power of APIs with simplicity and speed, right from your terminal. View OpenAPI documentations in your terminal.


## Ratatui - Hello World
{id: ratatui-hello-world}
{i: DefaultTerminal}
{i: render_widget}
{i: KeyEventKind}
{i: KeyCode}
{i: Paragraph}
{i: Event}

Taken from the [Hello world tutorial](https://ratatui.rs/tutorials/hello-world/)

* This example turns the terminal blue and writes some text on it.
* It waits till the user presses `q` and then quits.

![](examples/ratatui/hello-world/Cargo.toml)
![](examples/ratatui/hello-world/src/main.rs)


## Ratatui - handle keyboard events
{id: ratatui-handle-keyboard-events}
{i: KeyEventKind}
{i: KeyModifiers}
{i: CONTROL}
{i: SHIFT}
{i: ESC}


* We move the text to be displayed to a variable so we can change it.
* We accept every key on the keyboard.
* Pressing ESCape or Ctrl-C will terminate the application.
* Any other key or key-combination will be displayed on the screen. (e.g. Left, Right)

![](examples/ratatui/handle-the-keyboard/Cargo.toml)
![](examples/ratatui/handle-the-keyboard/src/main.rs)


## Ratatui - a more structured way to handle state
{id: ratatui-a-more-structured-way-to-handle-state}

* In this example we a struct, arbitrarily named `App` to repreent the state of the program. It will hold the text to be displayed and a flag indicating if we need to exit the program.
* We also separated the `draw` and the `handle_key_event` methods and moved the `run` method into the struct.
* We also implemented the `Widget`. Later we'll use a lot more widgets to improve the look of the application.

![](examples/ratatui/handle-the-keyboard-state/Cargo.toml)
![](examples/ratatui/handle-the-keyboard-state/src/main.rs)

## Ratatui - counting keyboard events
{id: ratatui-counting-keyboard-events}

* We have another field in the struct. A number that will be increased on ever press of the keyboard.

![](examples/ratatui/counting-keyboard-events/src/main.rs)

## Ratatui - a stopwatch or automatic counter - blocking vs. polling events
{id: ratatui-a-stopwatch-or-automatic-counter}
{i: poll}

* At first it still counts every keyboard even as the reading from the keyboard is a blocking call.
* We can use `event::poll` to check if there is anything to read at all. The `poll` has a timeout.
* This way we can do work (count) in-between checking for events.

![](examples/ratatui/stop-watch-counter/src/main.rs)

## Ratatui - counter
{id: ratatui-counter}

* A copy of the [counter app](https://ratatui.rs/tutorials/counter-app/)

* Thre is a crash if the counter overflows or underflows!

![](examples/ratatui/counter/Cargo.toml)
![](examples/ratatui/counter/src/main.rs)


## Ratatui - confirmation popup to verify intent to exit
{id: ratatui-confirmation-popup}
{i: TODO}

## Ratatui - widgets
{id: ratatui-widgets}
{i: TODO}

Show various widgets and how to work with them.

![](examples/ratatui/widgets/src/main.rs)




