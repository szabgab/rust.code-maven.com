# eGUI Window

* CentralPanel
* NativeOptions
* run_simple_native

```
cargo new egui-window
cd egui-window
cargo add eframe
```

This will make Cargo.toml look like this:

{% embed include file="src/examples/egui/egui-window/Cargo.toml" %}

The code is here:

{% embed include file="src/examples/egui/egui-window/src/main.rs" %}


{% embed include file="src/examples/egui/egui-window/screenshot.png)

When you click on the `X` to close the window, you might get lots of warnings like this:

```
warning: queue 0x7fa398000ca0 destroyed while proxies still attached:
```

This was [reported here](https://github.com/emilk/egui/issues/3413).


