use enumflags2::{BitFlag, BitFlags};

use crate::{McRead, McWrite};

impl<T: BitFlag<Numeric = N>, N: McRead> McRead for BitFlags<T, N> {
    type Args = N::Args;

    fn read(reader: impl std::io::Read, args: Self::Args) -> std::io::Result<Self> {
        let bits = N::read(reader, args)?;
        Ok(Self::from_bits_truncate(bits))
    }
}

impl<T: BitFlag<Numeric = N>, N: McWrite + Copy> McWrite for BitFlags<T, N> {
    type Args = N::Args;

    fn write(&self, writer: impl std::io::Write, args: Self::Args) -> std::io::Result<()> {
        self.bits().write(writer, args)
    }
}
