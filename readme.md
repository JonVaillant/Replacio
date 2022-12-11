# Replacio

Replace text in files, very quickly.

## Usage

Pass the following settings:

1. Search Directory path
2. Search Query to find
3. Replacement text
4. Ignore text case of files versus query

#### Binary Usage

```shell
./replacio ../../search-dir 'search phrase' 'replacement phrase' ignore-case
```

#### Dev Usage

```shell
cargo run -- ./search-dir 'search phrase' replacement ignore-case
```

## Watch Out!

Be careful with your inputs otherwise you may not get your data back. Example mistakes:

- Performing a replacement from storage volume's root (replaces text in all files on disk)
- Making a typo and running command (replace something else or something unexpected)

## Limitations

- Can only handle UTF-8 files.
- Not designed to handle multiple lines (yet).

## To-Do

- [x] Add test for replace
- [x] Test arguments containing spaces
- [ ] Move replace logic out of search_file
- [ ] Stop checking for match when replacing on matching results
- [ ] Handle matches spanning multiple lines
- [ ] Dry runs
