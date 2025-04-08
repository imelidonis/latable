# latable - LaTeX Table Generator

A simple CLI tool to generate tables for LaTeX. You can specify the number of rows
and columns and latable will generate a template table of that size. By default,
all columns are centered but you can specify the exact alignment of each column.

'latable' can also parse a csv file to generate the table. It generates a warning if
the csv file:

- has differnt number of  rows than what is spacified
- has rows with differnt number of columns

## Usage
```
Usage: latable [OPTIONS]

Options:
  -r, --rows <ROWS>           Number of rows
  -c, --columns <COLUMNS>     Number of columns
      --col-def <COLUMN_DEF>  Definition for each column. Provide either a rule for all columns or a
                              sequence with l (left-aligned) | c (center-aligned) | r (right-aligned)
                              for each column.
                              [possible values: centered, left, right, <sequence>] [default: centered]
      --csv <CSV_PATH>        Parse csv file to create the table. Must be comma-separated
  -h, --help                  Print help
  -V, --version               Print version
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

Table with 4 rows and 10 left-aligned columns. Instead of typing 'l'
for each column, you can pass the option 'left'.

```
$ latable -r 4 -c 10 --col-def left
LaTeX table generated:

\begin{table}[ht]
        \begin{tabular}{llllllllll}
                 & & & & & & & & & \\
                 & & & & & & & & & \\
                 & & & & & & & & & \\
                 & & & & & & & & &
        \end{tabular}
\end{table}
```

Parse csv file 'file.csv' and make the table left-aligned.

```
$ cat file.csv
c1,c2
value1,value2

$ latable --csv "./file.csv" --col-def left
LaTeX table generated:

\begin{table}[ht]
        \begin{tabular}{ll}
                c1 & c2 \\
                value1 & value2
        \end{tabular}
\end{table}
```
