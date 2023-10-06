# tin2stl

Convert TIN file to STL file.

## Install

```bash
cargo install tin2stl
```

## Usage

Command line:

```
tin2stl <source.tin>
```

Or Drop TIN file to tin2stl.exe in Windows.

Then you will get `source.tin.stl` in the same directory.

## Format of TIN file

TIN file format is a simple text file, each line is a triangle, each triangle has 3 points, each point has 3 coordinates.

```text
x1 y1 z1 x2 y2 z2 x3 y3 z3
```

For example:

```text
0 0 0 1 0 0 0 1 0
0 1 0 1 0 0 0 0 0
```

## License

MIT

## Author

[Usagi Ito/USAGI.NETWORK](https://usagi.network/)
