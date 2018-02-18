# Rutcher
A simple program which searches for a pattern in a file and patches it with a new one!

## Usage

```
$ rutcher ./patchme /tmp/patched 12345678 90909090
```

This will search for the bytes `0x12 0x34 0x56 0x78` and patch every occurrence with `0x90 0x90 0x90 0x90`.

