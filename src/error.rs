#[derive(Debug)]
pub enum ErrorType {
    InvalidHeader,
    OutOfIndex,
    Overflow,
    UnknownLayer,
    UnknownVersion,
    UnknownBitrate,
    UnknownFrequency,
    UnknownMode,
    BigValuesOutOfRange,
    BlockTypeForbidden,
}
