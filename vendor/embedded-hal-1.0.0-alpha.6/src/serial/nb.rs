//! Serial interface

/// Read half of a serial interface
///
/// Some serial interfaces support different data sizes (8 bits, 9 bits, etc.);
/// This can be encoded in this trait via the `Word` type parameter.
pub trait Read<Word = u8> {
    /// Read error
    type Error: crate::serial::Error;

    /// Reads a single word from the serial interface
    fn read(&mut self) -> nb::Result<Word, Self::Error>;
}

impl<T: Read<Word>, Word> Read<Word> for &mut T {
    type Error = T::Error;

    fn read(&mut self) -> nb::Result<Word, Self::Error> {
        T::read(self)
    }
}

/// Write half of a serial interface
pub trait Write<Word = u8> {
    /// Write error
    type Error: crate::serial::Error;

    /// Writes a single word to the serial interface
    fn write(&mut self, word: Word) -> nb::Result<(), Self::Error>;

    /// Ensures that none of the previously written words are still buffered
    fn flush(&mut self) -> nb::Result<(), Self::Error>;
}

impl<T: Write<Word>, Word> Write<Word> for &mut T {
    type Error = T::Error;

    fn write(&mut self, word: Word) -> nb::Result<(), Self::Error> {
        T::write(self, word)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        T::flush(self)
    }
}
