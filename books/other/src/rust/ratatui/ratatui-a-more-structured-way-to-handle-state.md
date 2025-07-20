# Ratatui - a more structured way to handle state

* In this example we a struct, arbitrarily named `App` to repreent the state of the program. It will hold the text to be displayed and a flag indicating if we need to exit the program.
* We also separated the `draw` and the `handle_key_event` methods and moved the `run` method into the struct.
* We also implemented the `Widget`. Later we'll use a lot more widgets to improve the look of the application.

{% embed include file="src/examples/ratatui/handle-the-keyboard-state/Cargo.toml" %}
{% embed include file="src/examples/ratatui/handle-the-keyboard-state/src/main.rs" %}



