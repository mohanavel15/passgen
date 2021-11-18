# passgen
Password Generator In Rust

## Help Command
```
$ passgen --help

Usage: passgen [-l <length>] [-a] [-c] [-n] [-s]

A password generator

Options:
  -l, --length      password length
  -a, --alphabets   switch for alphabets
  -c, --caps        switch for caps
  -n, --numbers     switch for numbers
  -s, --symbols     switch for symbols
  --help            display usage information
```

## Example

```
$ passgen -l 16 -a -c -n -s

&Rs~bNK9jWf2mToZ
```
