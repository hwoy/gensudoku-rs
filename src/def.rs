pub const NBLANK: u32 = 40;
pub const SD: u32 = 5;
pub const NBOARD: u32 = 1;

pub const HEAD_TEX: &'static str = r##"\documentclass[a4paper]{article}
\usepackage{geometry}
\usepackage{sudoku}
\begin{document}

"##;

pub const TAIL_TEX: &'static str = r##"

\end{document}
"##;

pub const HEAD_SUDOKU_TEX: &'static str = r##"\setlength\sudokusize{15cm}
\begin{sudoku}"##;

pub const TAIL_SUDOKU_TEX: &'static str = r##"\end{sudoku}"##;
