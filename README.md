# gensudoku-rs

gensudoku-rs is just a sudoku boards generator(TEX) in Rust.

## How to build

```sh

git clone https://github.com/hwoy/gensudoku-rs.git
cd gensudoku-rs
cargo build

```
## Example

```tex

\documentclass[a4paper]{article}
\usepackage{geometry}
\usepackage{sudoku}
\begin{document}


\noindent \verb|N_BLANKSEED = 2815627944, SBID = 2815627944| \verb|N = 0, SN_BLANK = 35, SD = 5| \newline \setlength\sudokusize{15cm}
\begin{sudoku}
| | | | |4| |3|1|6|.
|4|8| | | | |5| | |.
|1|6| |5|7| | | |4|.
|8| |6| | |3|7|5|9|.
|3|1|9| | |5|6|4| |.
|5| | |8|9| |1|3|2|.
|2| |8|1| | |9| |3|.
|6|9|1|3| | | | |5|.
|7|3|5| |6|2| | |1|.
\end{sudoku}
\newpage

\noindent \verb|N_BLANKSEED = 2815627945, SBID = 2815627945| \verb|N = 1, SN_BLANK = 36, SD = 5| \newline \setlength\sudokusize{15cm}
\begin{sudoku}
| |8|4|3| | |6|9| |.
|3|1|6| | |5| | |2|.
|9|7| |8| |2|3| |4|.
|8|6|2| | | | |5| |.
| | | |6| |4| |7| |.
| |4|3|2|5| | | |1|.
| |9| |4|2| | |3|7|.
|4| |7|5|8|6| |2|9|.
| |2| | |9|3|8|4| |.
\end{sudoku}

```

## Contact me

- Web: <https://github.com/hwoy>
- Email: <mailto:bosskillerz@gmail.com>
- Facebook: <https://www.facebook.com/watt.duean>


