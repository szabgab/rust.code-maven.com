# Conditional operation and boolean values in Rust
{id: boolean}

## Conditional: if
{id: rust-conditional-if}
{i: if}

![](examples/booleans/condition-if/src/main.rs)
![](examples/booleans/condition-if/out.out)

## Conditional: if - else
{id: rust-conditional-if-else}
{i: if}
{i: else}

![](examples/booleans/if-else/src/main.rs)
![](examples/booleans/if-else/out.out)

## Conditional: else - if
{id: rust-conditional-else-if}
{i: arms}
{i: else if}
{i: elseif}
{i: elsif}

* The code pathes in an if-else-if statement are called "arms".
* [comparison_chain](https://rust-lang.github.io/rust-clippy/master/index.html#comparison_chain)

![](examples/booleans/else-if/src/main.rs)
![](examples/booleans/else-if/out.out)

## Avoid the comparison chain using match
{id: avoid-the-comparison-chain}
{i: match}

![](examples/booleans/avoid-comparison-chain/src/main.rs)

## Rust: boolean values true and false
{id: rust-boolean-values}
{i: true}
{i: false}

![](examples/booleans/bool/src/main.rs)
![](examples/booleans/bool/out.out)

## Assign result of conditional to variable
{id: assign-result-to-variable}

![](examples/booleans/assign-result-to-variable/src/main.rs)
![](examples/booleans/assign-result-to-variable/out.out)

## Rust: other types don't have true/false values
{id: rust-boolean-only}

![](examples/booleans/other/src/main.rs)
![](examples/booleans/other/out.out)

* expected `bool`, found integer

## Toggle
{id: toggle}
{i: not}

* ! is the not-operator

![](examples/booleans/toggle/src/main.rs)
![](examples/booleans/toggle/out.out)

## if-else returns a value
{id: if-else-returns-value}

* This expersssion must have an `else` part!
* The last statement in both the `if` and the `else` part has no `;` at the end and thus they are called `expressions` and not `statements`.

![](examples/booleans/if-returns-value/src/main.rs)

## Conditional (Ternary) operator
{id: ternary-operator}
{i: ?:}
{i: if else}


![](examples/booleans/ternary/src/main.rs)
![](examples/booleans/ternary/out.out)

## match
{id: match}
{i: match}
{i: case}

* Similar to case or switch in other languages, `match` provides several `arms`.

![](examples/booleans/match-operator/src/main.rs)
![](examples/booleans/match-operator/out.out)

## match all the numbers of an integer type
{id: match-all-the-numbers-of-an-integer-type}

![](examples/booleans/integer-ranges/src/main.rs)

## match with conditions
{id: match-with-conditions}

![](examples/booleans/match-with-conditions/src/main.rs)
![](examples/booleans/match-with-conditions/out.out)

## Exercise: one-dimensional space-fight.
{id: exercise-one-dimensional-space-fight}

* We develop an interactive game called the one-dimensional space-fight.
* We have a spaceship in the one-dimensional space we need to shoot down.
* The distance of the spaceship is represented by an integer number. We can shoot by entering an integer number.

* The computer generates a random integer number between 0-20. (The distance of the spaceship.)
* The player shoots by entering a number.
* The computer tells us if our shot was too short, too long or if we hit the target.
* If we hit the target the game is over. We are told how many shots did we fire. We are asked if we would like to play again or quit.

* During the game we can press "c" (that stands for cheat) and the computer will reeal the distance of the spaceship.
* We can also press "q" and we will quite the game.
* We can also press "n" and we start a new game, giving up on the current fight.

## Solution: one-dimensional space-fight.
{id: solution-one-dimensional-space-fight}

![](examples/number-guessing-game/number-guessing-game/Cargo.toml)

![](examples/number-guessing-game/number-guessing-game/src/main.rs)


