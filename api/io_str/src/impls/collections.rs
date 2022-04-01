use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};
use std::io::{self, Read, Write};
use std::mem::MaybeUninit;

use minecrevy_io_buf::{ReadMinecraftExt, WriteMinecraftExt};

use crate::{options::*, McRead, McWrite};

impl McRead for String {
    type Options = StringOptions;

    fn read<R: Read>(mut reader: R, options: Self::Options) -> io::Result<Self> {
        // Read the len value and check upper bound
        let len = reader.read_var_i32_len()?;
        match options.max_len {
            Some(max_len) if len > max_len => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "exceeded max string length (max: {}, actual: {})",
                        max_len, len
                    ),
                ))
            }
            _ => {}
        }

        // Read the actual string as bytes
        let mut bytes = vec![0; len];
        reader.read_exact(&mut bytes)?;

        // Try to convert the bytes into valid UTF-8
        String::from_utf8(bytes).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "string has invalid UTF-8 characters",
            )
        })
    }
}

impl McWrite for String {
    type Options = StringOptions;

    fn write<W: Write>(&self, mut writer: W, options: Self::Options) -> io::Result<()> {
        match options.max_len {
            Some(max_len) if self.len() > max_len => return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("exceeded max string length (max: {}, actual: {})", max_len, self.len()),
            )),
            _ => {}
        }

        writer.write_var_i32_len(self.len())?;
        writer.write_all(self.as_bytes())?;
        Ok(())
    }
}

impl<T: McRead> McRead for Vec<T> {
    type Options = ListOptions<T::Options>;

    fn read<R: Read>(mut reader: R, options: Self::Options) -> io::Result<Self> {
        match options.length {
            ListLength::VarInt => {
                let len = reader.read_var_i32_len()?;
                let mut result = Vec::with_capacity(len);
                for _ in 0..len {
                    result.push(T::read(&mut reader, options.inner.clone())?);
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
                let mut result = Vec::with_capacity(len);
                for _ in 0..len {
                    result.push(T::read(&mut reader, options.inner.clone())?);
                }
                Ok(result)
            }
            ListLength::Remaining => {
                let mut result = Vec::new();
                loop {
                    match T::read(&mut reader, options.inner.clone()) {
                        Ok(v) => result.push(v),
                        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                        Err(e) => return Err(e),
                    }
                }
                Ok(result)
            }
        }
    }
}

impl<T: McWrite> McWrite for Vec<T> {
    type Options = ListOptions<T::Options>;

    fn write<W: Write>(&self, mut writer: W, options: Self::Options) -> io::Result<()> {
        match options.length {
            ListLength::VarInt => writer.write_var_i32_len(self.len())?,
            ListLength::Byte => {
                let len = i8::try_from(self.len())
                    .map_err(|_| io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("exceeded maximum list length: {}", self.len()),
                    ))?;
                writer.write_i8(len)?;
            }
            ListLength::Remaining => { /* no length prefix since its inferred */ }
        }
        for element in self {
            element.write(&mut writer, options.inner.clone())?;
        }
        Ok(())
    }
}

impl<K: McRead + Eq + Hash, V: McRead, S: BuildHasher + Default> McRead for HashMap<K, V, S> {
    type Options = ListOptions<(K::Options, V::Options)>;

    fn read<R: Read>(mut reader: R, options: Self::Options) -> io::Result<Self> {
        let (k, v) = options.inner;
        match options.length {
            ListLength::VarInt => {
                let len = reader.read_var_i32_len()?;
                let mut result = HashMap::with_capacity_and_hasher(len, S::default());
                for _ in 0..len {
                    result.insert(
                        K::read(&mut reader, k.clone())?,
                        V::read(&mut reader, v.clone())?,
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
                        K::read(&mut reader, k.clone())?,
                        V::read(&mut reader, v.clone())?,
                    );
                }
                Ok(result)
            }
            ListLength::Remaining => {
                let mut result = HashMap::with_hasher(S::default());
                loop {
                    match (
                        K::read(&mut reader, k.clone()),
                        V::read(&mut reader, v.clone()),
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

    fn write<W: Write>(&self, mut writer: W, options: Self::Options) -> io::Result<()> {
        let (k, v) = options.inner;
        match options.length {
            ListLength::VarInt => writer.write_var_i32_len(self.len())?,
            ListLength::Byte => {
                let len = i8::try_from(self.len())
                    .map_err(|_| io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("exceeded maximum list length: {}", self.len()),
                    ))?;
                writer.write_i8(len)?;
            }
            ListLength::Remaining => { /* no length prefix since its inferred */ }
        }
        for (key, value) in self {
            key.write(&mut writer, k.clone())?;
            value.write(&mut writer, v.clone())?;
        }
        Ok(())
    }
}

impl<T: McRead, const N: usize> McRead for [T; N] {
    type Options = ArrayOptions<T::Options>;

    fn read<R: Read>(mut reader: R, options: Self::Options) -> io::Result<Self> {
        let mut result = MaybeUninit::<T>::uninit_array::<N>();
        for val in &mut result {
            val.write(T::read(&mut reader, options.inner.clone())?);
        }
        // SAFETY: At this point, the array is always fully initialized,
        // otherwise we've already returned with an error.
        unsafe { Ok(MaybeUninit::array_assume_init(result)) }
    }
}

impl<T: McWrite, const N: usize> McWrite for [T; N] {
    type Options = ArrayOptions<T::Options>;

    fn write<W: Write>(&self, mut writer: W, options: Self::Options) -> io::Result<()> {
        for val in self {
            val.write(&mut writer, options.inner.clone())?;
        }
        Ok(())
    }
}

impl<T: McRead> McRead for Option<T> {
    type Options = OptionOptions<T::Options>;

    fn read<R: Read>(mut reader: R, options: Self::Options) -> io::Result<Self> {
        match options.existence {
            OptionExistence::Bool => {
                if reader.read_bool()? {
                    T::read(reader, options.inner).map(Some)
                } else {
                    Ok(None)
                }
            }
            OptionExistence::Remaining => match T::read(reader, options.inner) {
                Ok(v) => Ok(Some(v)),
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => Ok(None),
                Err(e) => return Err(e),
            },
        }
    }
}

impl<T: McWrite> McWrite for Option<T> {
    type Options = OptionOptions<T::Options>;

    fn write<W: Write>(&self, mut writer: W, options: Self::Options) -> io::Result<()> {
        match options.existence {
            OptionExistence::Bool => writer.write_bool(self.is_some())?,
            OptionExistence::Remaining => {}
        }

        if let Some(val) = self {
            val.write(writer, options.inner)?;
        }

        Ok(())
    }
}
