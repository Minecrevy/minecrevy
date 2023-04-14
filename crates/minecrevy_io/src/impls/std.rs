use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash},
    io::{self, Read, Write},
};

use crate::{
    options::{ListLength, ListOptions},
    std_ext::{ReadMinecraftExt, WriteMinecraftExt},
    McRead, McWrite, ProtocolVersion,
};

impl<K: McRead + Eq + Hash, V: McRead, S: BuildHasher + Default> McRead for HashMap<K, V, S> {
    type Options = ListOptions<(K::Options, V::Options)>;

    fn read<R: Read>(
        mut reader: R,
        options: Self::Options,
        version: ProtocolVersion,
    ) -> io::Result<Self> {
        let (k, v) = options.inner;
        match options.length {
            ListLength::VarInt => {
                let len = reader.read_var_i32_len()?;
                let mut result = HashMap::with_capacity_and_hasher(len, S::default());
                for _ in 0..len {
                    result.insert(
                        K::read(&mut reader, k.clone(), version)?,
                        V::read(&mut reader, v.clone(), version)?,
                    );
                }
                Ok(result)
            }
            ListLength::Byte => {
                let len = reader.read_i8()?;
                let len = usize::try_from(len).map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("invalid list length: {}", len),
                    )
                })?;
                let mut result = HashMap::with_capacity_and_hasher(len, S::default());
                for _ in 0..len {
                    result.insert(
                        K::read(&mut reader, k.clone(), version)?,
                        V::read(&mut reader, v.clone(), version)?,
                    );
                }
                Ok(result)
            }
            ListLength::Remaining => {
                let mut result = HashMap::with_hasher(S::default());
                loop {
                    match (
                        K::read(&mut reader, k.clone(), version),
                        V::read(&mut reader, v.clone(), version),
                    ) {
                        (Ok(k), Ok(v)) => {
                            result.insert(k, v);
                        }
                        (Err(e), _) | (_, Err(e)) if e.kind() == io::ErrorKind::UnexpectedEof => {
                            break
                        }
                        (Err(e), _) | (_, Err(e)) => return Err(e),
                    }
                }
                Ok(result)
            }
        }
    }
}

impl<K: McWrite, V: McWrite, S: BuildHasher> McWrite for HashMap<K, V, S> {
    type Options = ListOptions<(K::Options, V::Options)>;

    fn write<W: Write>(
        &self,
        mut writer: W,
        options: Self::Options,
        version: ProtocolVersion,
    ) -> io::Result<()> {
        let (k, v) = options.inner;
        match options.length {
            ListLength::VarInt => writer.write_var_i32_len(self.len())?,
            ListLength::Byte => {
                let len = i8::try_from(self.len()).map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("exceeded maximum list length: {}", self.len()),
                    )
                })?;
                writer.write_i8(len)?;
            }
            ListLength::Remaining => { /* no length prefix since its inferred */ }
        }
        for (key, value) in self {
            key.write(&mut writer, k.clone(), version)?;
            value.write(&mut writer, v.clone(), version)?;
        }
        Ok(())
    }
}
