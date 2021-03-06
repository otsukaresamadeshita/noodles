/// A FASTA index record field.
#[derive(Clone, Copy, Debug)]
pub enum Field {
    /// The reference sequence name.
    ReferenceSequenceName,
    /// The total length of the sequence.
    Length,
    /// The offset from the start.
    Offset,
    /// The number of bases in a line.
    LineBases,
    /// The number of characters in a line.
    LineWidth,
}
