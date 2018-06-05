# Assert, a toy project in Rust 

`assert` is a small project written in **Rust**. The main idea here is provide a
cli with 2 sub-commands, `eq` and `diff`. Given 2 strings `assert` will compare
and return 0 if condition succeed or 1 if condition fails. 

Both sub-commands can receive the `print` argument, so instead return 0 or 1 it
will print a nicely and friendly message. 

## Eq

Running `assert` with `-h` or `--help` will show both sub-commands, `eq` and
`diff`. 

```
$ ./assert -h
assert 1.0.0
Marcelo Castellani <marcelofc.rock@gmail.com>
Compare two strings and return

USAGE:
    assert [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    diff    Compare both values. If not equal return 0, otherwise 1.
    eq      Compare both values. If equal return 0, otherwise 1.
    help    Prints this message or the help of the given subcommand(s)
```

Let's take a look at `eq` command.

```
$ ./assert eq "First string" "Second String" --print
Checking if First string is equal to Second String
Both are not equal
```

Using the `print` flag you will see the result of comparison. You can also use
return code (that's why I thought this `assert` tool is useful).

```
$ ./assert eq "First string" "Second String"
$ echo $?
1
```

If strings are not equal you got 1. If both are equal you got 0:

```
$ ./assert eq "The string" "The string"
$ echo $?
0
```

## Diff

`diff` is the opposite of `eq`. If strings are not equal you got 0, otherwise
you got 1.

```
$ ./assert diff "The string" "The string"
$ echo $?
1
```


