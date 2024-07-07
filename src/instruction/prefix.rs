use number;

pub enum EscapeSize {
    Byte,
    Word
}

#[repr(u8)]
pub enum Prefix {
    /// Escape code to read specific number of bytes that will be interpreted as the operation code.
    Escape(EscapeSize),
    /// Synchronize execution among other processors.
    Synchronize,
    /// Set the operands segment size. This determines how large the data for computation is.
    DataSize(number::Size)
}