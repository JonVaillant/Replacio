# Replacio

Replace text in files, very quickly.


## Usage

Pass the following settings:

1. Search Directory path
2. Search Query to find
3. Replacement text
4. Flags
    - `ignore-case` to ignore text case (uppercase VS lowercase)
    - `dry` to search and not replace

#### Binary Usage

After compiling the binary, run it:

```shell
./replacio ../../search-dir 'search phrase' 'replacement phrase' ignore-case
```

#### Dev Usage

With Rust & Cargo installed run:

```shell
cargo run -- ./search-dir 'search phrase' replacement dry ignore-case
```


## Watch Out!

Be careful with your inputs otherwise you may not get your data back. Example mistakes:

- Performing a replacement from storage volume's root (replaces text in all files on disk)
- Making a typo and running command (replace something else or something unexpected)


## Limitations

- Can only handle UTF-8 files.
- Not designed to handle multiple lines (yet).


## Testing

```shell
cargo test
```


## To-Do

- [x] Add test for replace
- [x] Test arguments containing spaces
- [x] Move replace logic out of search_file
- [ ] Handle matches spanning multiple lines
- [x] Dry runs

