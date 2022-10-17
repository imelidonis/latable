// Column Definition Options.
use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum ColumnDef {
    /// Centered Columns
    Centered,

    /// Left-aligned Columns
    Left,

    /// Right-aligned Columns
    Right,

    /// User-defined Column Alignment Sequence
    /// [Accepted options: l (left-aligned), c (center-aligned), r (right-aligned)]
    #[value(skip)]
    Custom(String),
}

impl std::fmt::Display for ColumnDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}
