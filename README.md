# Type racer

Type racer is a single player game, created with the [ggez](https://ggez.rs/) framework for Rust.

The idea here is a little bit different from the classic game. Words are comming from the left side of the screen and go to the right. The player should write the incoming words without typo and earn points. If you leave a word to get to the right side of the screen you lose 1 life. If you lose all of your lifes -> Game Over :/\. The question is how far can you get and how much points you can earn? :)

## Buffs && Nurfs

### Buffs:
- instant remove 5 words from the screen
- add extra 1 life
- slow down the words
- autocomplete with 1 char (for example)

### Nerfs:
- speed-up the words
- color changing words
- screen brightness pulsing