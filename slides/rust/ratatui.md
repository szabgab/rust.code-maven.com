# Ratatui
{id: ratatui}

## What is Ratatui?
{id: what-is-ratatui}

* TUI = Terminal User Interface
* [Ratatui](https://ratatui.rs/)

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

## Ratatui - a stopwatch or automatic counter
{id: ratatui-a-stopwatch-or-automatic-counter}
{i: TODO}

## Ratatui - blocking vs. polling events
{id: ratatui-blocking-vs-polling-events}


## Ratatui - counter
{id: ratatui-counter}

* A copy of the [counter app](https://ratatui.rs/tutorials/counter-app/)

* Thre is a crash if the counter overflows or underflows!

![](examples/ratatui/counter/Cargo.toml)
![](examples/ratatui/counter/src/main.rs)

