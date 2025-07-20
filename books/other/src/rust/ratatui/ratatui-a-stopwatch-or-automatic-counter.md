# Ratatui - a stopwatch or automatic counter - blocking vs. polling events

* poll

* At first it still counts every keyboard even as the reading from the keyboard is a blocking call.
* We can use `event::poll` to check if there is anything to read at all. The `poll` has a timeout.
* This way we can do work (count) in-between checking for events.

{% embed include file="src/examples/ratatui/stop-watch-counter/src/main.rs" %}


