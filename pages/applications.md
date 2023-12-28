---
title: Open Source Applications written in Rust
timestamp: 2023-12-26T18:30:01
published: true
description: One of the best ways to learn how to write end-user applications in Rust is looking at such applications.
tags:
    - applications
---

If someone wants to learn how to create a Rust library it is very easy to find [Crates](https://crates.io/) and see how other people do things.

However, our real goal is to create applications, or end-user applications that serve other people as well. People who most likely are not Rust programmers.

One of the best ways to learn how to write end-user applications in Rust is looking at such applications. So on this page I'll collect a few
and as time permits I'll take a look at them and write articles about them.

## GUI Applications

* [AppFlowy](https://appflowy.io/) (Flutter via custom ffi bridge, according to the Redditor)
* [Gyroflow](https://gyroflow.xyz/) (QML via qmetaobject-rs)
* [Makepad](https://makepad.nl/) See  the [Makepad Studio](https://makepad.dev/)
* [Lapce](https://lapce.dev/) Lightning-fast and Powerful Code Editor  (Floem)
* [OctaSine](https://www.octasine.com/) written using [Iced](https://iced.rs/)
* [Psst](https://github.com/jpochyla/psst) written in [Druid](https://linebender.org/druid/)
* [Rerun](https://www.rerun.io/) written in [egui](https://www.egui.rs/)
* [Sniffnet](https://sniffnet.net/) written using [Iced](https://iced.rs/)
* [Spacedrive](https://www.spacedrive.com/) (Tauri)
* [Loopers](https://github.com/mwylde/loopers) is a graphical live looper.
* [Helix text editor](https://helix-editor.com/)
* [Perspective](https://perspective.finos.org/) written in [Yew](https://yew.rs/).
* [FontFinder](https://github.com/mmstick/fontfinder) GTK application for browsing and installing fonts from Google's font archive.
* [ruffle](https://ruffle.rs/) is a Flash Player emulator built in the Rust programming language.
* [Czkawka](https://github.com/qarmin/czkawka/) Multi functional app to find duplicates, empty folders, similar images etc. Both CLI and GUI using GTK 4 or Slint.
* [Warp](https://apps.gnome.org/Warp/) Fast and secure file transfer written in [GTK.rs](https://gtk-rs.org/).

## Terminal UI applications

* Showcasing [terminal UI apps](https://ratatui.rs/showcase/apps/) and a longer list of [Awesome applications](https://github.com/ratatui-org/awesome-ratatui#-apps) using [Ratatui](https://ratatui.rs/).

* [Fish](https://fishshell.com/) is a smart and user-friendly command line shell for Linux, macOS, and the rest of the family. It is [almost done rewriting in Rust.](https://github.com/fish-shell/fish-shell/discussions/10123).



## Command Line Tools

* [mdBook](https://rust-lang.github.io/mdBook/) is a command line tool to create books with Markdown.
* [Zola](https://www.getzola.org/) one-stop static site engine.
* [Cobalt](https://cobalt-org.github.io/) A static-site generator that works for you.
* [ripgrep](https://github.com/BurntSushi/ripgrep)
* [Trippy](https://trippy.cli.rs/) combines the functionality of traceroute and ping and is designed to assist with the analysis of networking issues.
* [BOS CLI](https://github.com/bos-cli-rs/bos-cli-rs) a command line utility that simplifies local component development for NEAR BOS.
* [carl](https://carl.cli.rs/) is a calendar for the commandline. It tries to mimic the various cal(1) implementations out there, but also adds enhanced features like colors and ical support.
* [Colmena](https://colmena.cli.rs/) is a simple, stateless NixOS deployment tool modeled after NixOps and morph.

* [Rust Tools apt repo](https://apt.cli.rs/) this is only a README file. I don't know [where is the source](https://github.com/ethanhs/apt.cli.rs/issues/14) of the rest of the site.

* There are a number of projects hosted as subdomains of [CLI.rs](https://cli.rs/). Right now the only way to see the list is to look in the [repository](https://github.com/zackify/cli.rs/tree/master/domains)


## Other

* [Brainease](https://brainease.cli.rs/) is a scripting language that has many similar concepts with brainf-ck, sharing some principal concepts, like memory and instructions.

* [Moonfire NVR](https://github.com/scottlamb/moonfire-nvr) is an open-source security camera network video recorder. I personally don't have a lot of experience with hardware projects but I would like to get involved in some.


The [Awesome Rust](https://github.com/rust-unofficial/awesome-rust) is a huge list of projects, some of them are applications. That would be a good source.

There is also the [Awesome Alternatives in Rust](https://github.com/TaKO8Ki/awesome-alternatives-in-rust).

Several of these were collected via [this Reddit thread](https://www.reddit.com/r/rust/comments/18rumat/open_source_enduser_applications_written_in_rust/)
