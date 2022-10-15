# latable - LaTeX Table Generator

A simple CLI tool to generate tables for LaTeX. You can specify the number of rows
and columns and latable will generate a template table of that size. By default,
all columns are centered but you can specify the exact alignment of each column.

## Usage
```
Usage: latable [OPTIONS] --rows <ROWS> --columns <COLUMNS>

Options:
  -r, --rows <ROWS>           Number of rows
  -c, --columns <COLUMNS>     Number of columns
      --col-def <COLUMN_DEF>  Column Definition, eg. 'ccc' for 3 centered columns.
                              Default: all columns centered
                              Accepted options: l (left-aligned), c (center-aligned), r (right-aligned).
  -h, --help                  Print help information
  -V, --version               Print version information
```

## Examples

Table with 4 rows and 5 centered columns:

```
$ latable -r 4 -c 5
LaTeX table generated:

\begin{table}[ht]
    \begin{tabular}{ccccc}
         & & & & \\
         & & & & \\
         & & & & \\
         & & & & 
    \end{tabular}
\end{table}
```

Table with 4 rows and 5 columns with the first two being left-aligned.

```
$ latable -r 4 -c 5 --col-def llccc
LaTeX table generated:

\begin{table}[ht]
    \begin{tabular}{llccc}
         & & & & \\
         & & & & \\
         & & & & \\
         & & & & 
    \end{tabular}
\end{table}

```

