# Type racer

Type racer is a single player game, created with the [ggez](https://ggez.rs/) framework for Rust.

The idea here is a little bit different from the classic game. Words are comming from the left side of the screen and go to the right. The player should write the incoming words without typo and earn points. If you leave a word to get to the right side of the screen you lose 1 life. If you lose all of your lifes -> Game Over :/\. The question is how far can you get and how much points you can earn? :)

## Buffs && Nurfs

### Buffs:
- instant random words removal from the screen
- extra 1 life
- slow down the word spawn
- autocomplete with 1 char (?)

### Nerfs:
- speed-up the words over time
- color changing words
- screen shaking

## Scoreboard
The scoreboard is haved in the user home directory.

For Linux:
```
~/.config/type_racer/scoring.data
```

## Installation

*required [rustc with cargo](https://rustup.rs/) to be installed*

- command for installing ggez dependent packages

```
sudo apt install libasound2-dev libudev-dev pkg-config
```

## Compiling and starting the game

- normal
```
cargo run
```
- debug mode
```
DEBUG=1 cargo run
```

- release mode
```
cargo run --release
```
