# grrs
Command line tool which takes a pattern and a path to a file and returns all the lines which that pattern occurred in the file.

## Example
`example.txt`
```txt
hello
world
welcome
to
rust
we
are
here
to
learn
```

Running on the command line:

```shell
> grrs we ./example.txt
> welcome
> we
```