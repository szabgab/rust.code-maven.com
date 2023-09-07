---
title: The feedback loop for learning Rust
timestamp: 2023-09-07T07:30:01
description:
tags:
    - rustc
    - vim
todo:
    - TODO
---

One of the most important tools for learning anything is the feedback loop. If you don't get feedback about your work, about your performance it almost impossible to improve.

In this article we'll go over some of the feedback loops for learning Rust.

Some of them you have without asking. Some you have to activate yourself and for some you have to actively seek out.
Some are automated and thus are very fast, some need external human interaction and thus will take more time, but can be more insightful.


## The Rust compiler

As you try to run your code first you will need to compile it. Even if you use `cargo run`, behind the scenes `rustc`, the Rust compiler first compiles the code.
The Rust compiler is famous for being very picky. It will complain about a lot of things. Some would say don't take it personally, but it might be better if you do:
The Rust compiler personally helps you to become a better Rust developer.

I know all those errors and warnings are annoying, but at the end it is for your own good.

For this you don't need to do anything in particular. Just try to compile or run your code.

## Editors and IDEs

Even before you try to compile and run your code, some editors and IDEs will already tell you when something is wrong. They actually use the Rust compiler and maybe some other tools behind the scenes to accomplish this, but from point of view, you get the feedback while writing the code.

In order to get this feedback you need to use one of the editors or IDEs that have integration with the [rust-analyzer](https://rust-analyzer.github.io/).

I personally use [Visual Studio Code](https://code.visualstudio.com/docs/languages/rust) with the [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

I also use [vim](https://www.vim.org/) a lot. There is a [vim plugin for Rust](https://github.com/rust-lang/rust.vim) you can configure.

## cargo fix

Not only will the compile complain, but you can even ask it to fix your code. Run

```
cargo fix
```

and rust will try to fix all the things it was complaining about. You can then look at the changes and learn from them.


## Looking at the results manually

This is not a tool, but I felt I should mention it so we can appreciate it. When you are learning a human language (e.g. Spanish or English) and you write down a sentence, you need another human to check it grammatically and give you feedback if it was a correct and meaningful sentence. There is a growing number of tools that will help you with this in human languages, but with programming it is given. The compile will tell you if your code is grammatically correct and you can always look at the result to verify that the results are what you expected.

## Writing functional tests

You can go a step further and automate the verification of the results by writing unit-, integration-, and other **functional tests**. It is especially valuable for multi-stage exercises when you get an assignment and then the next assignment, the next exercise builds on the previous one. These exercises can imitate the real world scenarios better than one-off assignments. After all in the real world usually we will have to change our existing code to add new features. Not only will writing test help with your exercises, they will also help you when you start writing production code and you have already learned how to write tests.

[The Rust book](https://doc.rust-lang.org/book/) has a whole chapter about writing tests.

You will be able to run your tests by typing in:

```
cargo test
```

## Formatting cargo fmt

Not exactly a feedback tool, but having consistently formatted code is very valuable. So it is better getting used to it soon. Rust provides the [rustfmt](https://github.com/rust-lang/rustfmt) that, if not installed yet you can easily install and then run with the `cargo fmt` command. It will reformat your code.

If you only want it to report the things it would want to reformat you can run it with the `check` flag as in

```
cargo fmt --check
```

If you don't like the default formatting rules, you can configure it.


## Clippy

Rust also has a very good linter called [Clippy](https://github.com/rust-lang/rust-clippy). If it is not installed yet you can install it and then run it as

```
cargo clippy
```

It will give a lot of things you could improve in your code. You can go over them one-by-one, learn from each issue and fix them.

Alternatively you could ask Clippy to try to fix them for you by using the `fix` flag:

```
cargo clippy --fix
```

## Rustlings

[Rustlings](https://github.com/rust-lang/rustlings) are a set of exercises (around 100 when I last checked). You get the description of the exercise and the automated test verifying your solution. You need to implement the assignment, but you already get the feedback of the tests.

## Exercism

[Exercism](https://exercism.org/) is a platform for practicing programming languages. It had 67 language tracks when I last checked, one of them is Rust. Similar to Rustling (though I guess Exercism came first), you get assignments and the functional tests that help you verify the correctness of your solution. That, however is only part of the value. You can then submit your solution and volunteers will check your implementation and make suggestions how to improve it. Of course it can take hours, sometimes even days or weeks to get feedback as there are people involved in the process. People who volunteer in their free time to help others.

## Rust user forum

There are quite a few channels where you can discuss Rust on various levels. Some are for the people who develop the compiler, some for people who develop crates and others for people who just write Rust. Though I like to thing about them as future Crate developers and future contributors to the rust compiler, but for now, let's just call them Rust users. For example there is the [Rust User forum](https://users.rust-lang.org/). Once you sign up you can create a "New topic" and put the "code review" category on it. Then paste your code with some explanation and ask for feedback. Within some time you will get some very nice comments. I tried it a few times and in each case within a few hours my code was improved and I learned a lot of new things.

## ChatGPT

I am really not sure how good it is. I tried one of the examples for which I got a really nice improvement in the Rust user forum, but ChatGPT could only do minor improvements and I am not even sure if one of those wasn't making my code worse. Maybe if I had access to version 4 it would be much better, I don't know.


## Help others

Ultimately, probably the best way to learn is to try to explain things to others. You could write [blog posts about Rust](/) as I do. You could turn the table around and become a mentor on [Exercism](https://exercism.org/) giving feedback to others. You could start monitoring the [Rust User forum](https://users.rust-lang.org/) or [Rust on StackOverflow](https://stackoverflow.com/tags/rust) and try to help people. Not only will you get the extremely good feeling of helping someone, but you will also learn a lot. Both about Rust and yourself.

