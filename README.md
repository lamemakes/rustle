<p align="center">
    <img src="docs/images/rustle_logo.png" alt="Rustle Logo" width="350px"/>
</p>
<p align="center">An open source Wordle CLI built in Rust</p>

## What is Rustle?
 
Rustle allows you to play the daily Wordle ad-free and from the comfort of your favorite Command Line Interface (CLI) - Unix or Windows!

Don't have a network connection or want to do some random Wordles? No worries - Rustle gives you the option to play a randomly generated Wordle based on an existing wordlist!

## Build & Install

The following tutorial shows you how to build Rustle:
```
git clone git@github.com:lamemakes/rustle.git   # Clone Rustle
cd rustle                                       # Enter the Rustle dir
cargo build                                     # Build Rustle
```

To play using the NYT's Wordle solution of the day:
```
./target/release/rustle                         # Run "online" Rustle
```

or, for a random "offline" Wordle solution:
```
./target/release/rustle --offline
```


## How to Play

- All guesses must be five letters long
- Your guess must be in the NYT's [list of valid words](https://raw.githubusercontent.com/lamemakes/rustle/master/src/assets/wordlist.json)
- The color of the tiles will change based on how close your guess was

### Examples

![correct_letter](docs/images/correct_letter.png)
<br>**C** is in the word and is in the correct spot.

![misplaced_letter](docs/images/misplaced_letter.png)
<br>**R** is in the word but in the wrong spot.

![incorrect_letter](docs/images//incorrect_letter.png)
<br>None of these letters are in the word.

### Gameplay

<img src="docs/images/rustle_win.gif" width="50%">