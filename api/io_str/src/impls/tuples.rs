use std::io::{self, Read, Write};

use crate::{McRead, McWrite};

impl McRead for () {
    type Options = ();

    fn read<R: Read>(_reader: R, (): Self::Options) -> io::Result<Self> {
        Ok(())
    }
}

impl McWrite for () {
    type Options = ();

    fn write<W: Write>(&self, _writer: W, (): Self::Options) -> io::Result<()> {
        Ok(())
    }
}

impl<A: McRead> McRead for (A,) {
    type Options = (A::Options,);

    fn read<R: Read>(mut reader: R, (a,): Self::Options) -> io::Result<Self> {
        Ok((A::read(&mut reader, a)?,))
    }
}

impl<A: McWrite> McWrite for (A,) {
    type Options = (A::Options,);

    fn write<W: Write>(&self, mut writer: W, (a,): Self::Options) -> io::Result<()> {
        self.0.write(&mut writer, a)?;
        Ok(())
    }
}

impl<A: McRead, B: McRead> McRead for (A, B) {
    type Options = (A::Options, B::Options);

    fn read<R: Read>(mut reader: R, (a, b): Self::Options) -> io::Result<Self> {
        Ok((A::read(&mut reader, a)?, B::read(&mut reader, b)?))
    }
}

impl<A: McWrite, B: McWrite> McWrite for (A, B) {
    type Options = (A::Options, B::Options);

    fn write<W: Write>(&self, mut writer: W, (a, b): Self::Options) -> io::Result<()> {
        self.0.write(&mut writer, a)?;
        self.1.write(&mut writer, b)?;
        Ok(())
    }
}

impl<A: McRead, B: McRead, C: McRead> McRead for (A, B, C) {
    type Options = (A::Options, B::Options, C::Options);

    fn read<R: Read>(mut reader: R, (a, b, c): Self::Options) -> io::Result<Self> {
        Ok((
            A::read(&mut reader, a)?,
            B::read(&mut reader, b)?,
            C::read(&mut reader, c)?,
        ))
    }
}

impl<A: McWrite, B: McWrite, C: McWrite> McWrite for (A, B, C) {
    type Options = (A::Options, B::Options, C::Options);

    fn write<W: Write>(&self, mut writer: W, (a, b, c): Self::Options) -> io::Result<()> {
        self.0.write(&mut writer, a)?;
        self.1.write(&mut writer, b)?;
        self.2.write(&mut writer, c)?;
        Ok(())
    }
}

impl<A: McRead, B: McRead, C: McRead, D: McRead> McRead for (A, B, C, D) {
    type Options = (A::Options, B::Options, C::Options, D::Options);

    fn read<R: Read>(mut reader: R, (a, b, c, d): Self::Options) -> io::Result<Self> {
        Ok((
            A::read(&mut reader, a)?,
            B::read(&mut reader, b)?,
            C::read(&mut reader, c)?,
            D::read(&mut reader, d)?,
        ))
    }
}

impl<A: McWrite, B: McWrite, C: McWrite, D: McWrite> McWrite for (A, B, C, D) {
    type Options = (A::Options, B::Options, C::Options, D::Options);

    fn write<W: Write>(&self, mut writer: W, (a, b, c, d): Self::Options) -> io::Result<()> {
        self.0.write(&mut writer, a)?;
        self.1.write(&mut writer, b)?;
        self.2.write(&mut writer, c)?;
        self.3.write(&mut writer, d)?;
        Ok(())
    }
}

impl<A: McRead, B: McRead, C: McRead, D: McRead, E: McRead> McRead for (A, B, C, D, E) {
    type Options = (A::Options, B::Options, C::Options, D::Options, E::Options);

    fn read<R: Read>(mut reader: R, (a, b, c, d, e): Self::Options) -> io::Result<Self> {
        Ok((
            A::read(&mut reader, a)?,
            B::read(&mut reader, b)?,
            C::read(&mut reader, c)?,
            D::read(&mut reader, d)?,
            E::read(&mut reader, e)?,
        ))
    }
}

impl<A: McWrite, B: McWrite, C: McWrite, D: McWrite, E: McWrite> McWrite for (A, B, C, D, E) {
    type Options = (A::Options, B::Options, C::Options, D::Options, E::Options);

    fn write<W: Write>(&self, mut writer: W, (a, b, c, d, e): Self::Options) -> io::Result<()> {
        self.0.write(&mut writer, a)?;
        self.1.write(&mut writer, b)?;
        self.2.write(&mut writer, c)?;
        self.3.write(&mut writer, d)?;
        self.4.write(&mut writer, e)?;
        Ok(())
    }
}
