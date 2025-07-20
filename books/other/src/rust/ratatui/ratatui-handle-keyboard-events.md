# Ratatui - handle keyboard events

* KeyEventKind
* KeyModifiers
* CONTROL
* SHIFT
* ESC


* We move the text to be displayed to a variable so we can change it.
* We accept every key on the keyboard.
* Pressing ESCape or Ctrl-C will terminate the application.
* Any other key or key-combination will be displayed on the screen. (e.g. Left, Right)

{% embed include file="src/examples/ratatui/handle-the-keyboard/Cargo.toml" %}
{% embed include file="src/examples/ratatui/handle-the-keyboard/src/main.rs" %}


