# uecho - Like the echo for linux but unicode
I created this mostly because my current setup made it difficult for me to insert unicode characters in my text editor.
This takes arguments as a decimal or hexadecimal unicode number and echo it's corresponding unicode character. If none exist it prints the replacement character �.

```sh
$> uecho 0x2665 0x1f980
♥ 🦀
```

# Installation

Run the command `cargo install uecho`

# Usage

uecho reads input and expects only decimals and hexadecimal numbers by default.

```sh
$> uecho 0x2665 0x1f980
♥ 🦀
```

While

```sh
$> uecho 0x2665 test 0x1f980
♥ � 🦀
```
**test** is not a valid number for a unicode character and a replacement character is therefore replaced. If you wish to keep the original input if an invalid unicode number is encountered add the argument *-e* to the command.

```sh
$> uecho -e 0x2665 test 0x1f980
♥ test 🦀
```
