use std::io;

use byteorder::{BigEndian, ReadBytesExt as _, WriteBytesExt as _};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Value {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(List),
    Compound(Compound),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Value {
    fn tag(&self) -> Tag {
        match self {
            Self::Byte(_) => Tag::Byte,
            Self::Short(_) => Tag::Short,
            Self::Int(_) => Tag::Int,
            Self::Long(_) => Tag::Long,
            Self::Float(_) => Tag::Float,
            Self::Double(_) => Tag::Double,
            Self::ByteArray(_) => Tag::ByteArray,
            Self::String(_) => Tag::String,
            Self::List(_) => Tag::List,
            Self::Compound(_) => Tag::Compound,
            Self::IntArray(_) => Tag::IntArray,
            Self::LongArray(_) => Tag::LongArray,
        }
    }

    fn read_tag(reader: &mut impl io::Read) -> io::Result<Tag> {
        match reader.read_u8()? {
            0 => Ok(Tag::End),
            1 => Ok(Tag::Byte),
            2 => Ok(Tag::Short),
            3 => Ok(Tag::Int),
            4 => Ok(Tag::Long),
            5 => Ok(Tag::Float),
            6 => Ok(Tag::Double),
            7 => Ok(Tag::ByteArray),
            8 => Ok(Tag::String),
            9 => Ok(Tag::List),
            10 => Ok(Tag::Compound),
            11 => Ok(Tag::IntArray),
            12 => Ok(Tag::LongArray),
            t => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid tag type {t}"),
            )),
        }
    }

    fn read_value(reader: &mut impl io::Read, tag: Tag) -> io::Result<Self> {
        match tag {
            Tag::End => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "unexpected end tag",
            )),
            Tag::Byte => Self::read_byte(reader).map(Self::Byte),
            Tag::Short => Self::read_short(reader).map(Self::Short),
            Tag::Int => Self::read_int(reader).map(Self::Int),
            Tag::Long => Self::read_long(reader).map(Self::Long),
            Tag::Float => Self::read_float(reader).map(Self::Float),
            Tag::Double => Self::read_double(reader).map(Self::Double),
            Tag::ByteArray => Self::read_byte_array(reader).map(Self::ByteArray),
            Tag::String => Self::read_string(reader).map(Self::String),
            Tag::List => Self::read_list(reader).map(Self::List),
            Tag::Compound => Self::read_component(reader).map(Self::Compound),
            Tag::IntArray => Self::read_int_array(reader).map(Self::IntArray),
            Tag::LongArray => Self::read_long_array(reader).map(Self::LongArray),
        }
    }

    fn read_byte(reader: &mut impl io::Read) -> io::Result<i8> {
        reader.read_i8()
    }

    fn read_short(reader: &mut impl io::Read) -> io::Result<i16> {
        reader.read_i16::<BigEndian>()
    }

    fn read_int(reader: &mut impl io::Read) -> io::Result<i32> {
        reader.read_i32::<BigEndian>()
    }

    fn read_long(reader: &mut impl io::Read) -> io::Result<i64> {
        reader.read_i64::<BigEndian>()
    }

    fn read_float(reader: &mut impl io::Read) -> io::Result<f32> {
        reader.read_f32::<BigEndian>()
    }

    fn read_double(reader: &mut impl io::Read) -> io::Result<f64> {
        reader.read_f64::<BigEndian>()
    }

    fn read_byte_array(reader: &mut impl io::Read) -> io::Result<Vec<i8>> {
        let len = reader.read_i32::<BigEndian>()?;
        if len.is_negative() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "byte array length is negative",
            ));
        }
        let mut buf = vec![0; len as usize];
        reader.read_exact(&mut buf)?;
        Ok(buf.into_iter().map(|b| b as i8).collect())
    }

    fn read_string(reader: &mut impl io::Read) -> io::Result<String> {
        let len = reader.read_u16::<BigEndian>()?;
        let mut buf = vec![0; len as usize];
        reader.read_exact(&mut buf)?;

        String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    fn read_list(reader: &mut impl io::Read) -> io::Result<List> {
        match Self::read_tag(reader.by_ref())? {
            Tag::End => match reader.read_i32::<BigEndian>()? {
                1 => Ok(List::End),
                len => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("invalid end tag list length {len}"),
                )),
            },
            Tag::Byte => Ok(List::Byte(Self::read_list_of(
                reader.by_ref(),
                true,
                |r| Self::read_byte(r),
            )?)),
            Tag::Short => Ok(List::Short(Self::read_list_of(
                reader.by_ref(),
                true,
                |r| Self::read_short(r),
            )?)),
            Tag::Int => Ok(List::Int(Self::read_list_of(reader.by_ref(), true, |r| {
                Self::read_int(r)
            })?)),
            Tag::Long => Ok(List::Long(Self::read_list_of(
                reader.by_ref(),
                true,
                |r| Self::read_long(r),
            )?)),
            Tag::Float => Ok(List::Float(Self::read_list_of(
                reader.by_ref(),
                true,
                |r| Self::read_float(r),
            )?)),
            Tag::Double => Ok(List::Double(Self::read_list_of(
                reader.by_ref(),
                true,
                |r| Self::read_double(r),
            )?)),
            Tag::ByteArray => Ok(List::ByteArray(Self::read_list_of(
                reader.by_ref(),
                false,
                |r| Self::read_byte_array(r),
            )?)),
            Tag::String => Ok(List::String(Self::read_list_of(
                reader.by_ref(),
                false,
                |r| Self::read_string(r),
            )?)),
            Tag::List => Ok(List::List(Self::read_list_of(
                reader.by_ref(),
                false,
                |r| Self::read_list(r),
            )?)),
            Tag::Compound => Ok(List::Compound(Self::read_list_of(
                reader.by_ref(),
                false,
                |r| Self::read_component(r),
            )?)),
            Tag::IntArray => Ok(List::IntArray(Self::read_list_of(
                reader.by_ref(),
                false,
                |r| Self::read_int_array(r),
            )?)),
            Tag::LongArray => Ok(List::LongArray(Self::read_list_of(
                reader.by_ref(),
                false,
                |r| Self::read_long_array(r),
            )?)),
        }
    }

    fn read_list_of<T, R: io::Read>(
        reader: &mut R,
        has_exact_size: bool,
        mut read: impl FnMut(&mut R) -> io::Result<T>,
    ) -> io::Result<Vec<T>> {
        let len = reader.read_i32::<BigEndian>()?;
        if len.is_negative() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "list length is negative",
            ));
        }

        let mut buf = Vec::with_capacity(if has_exact_size { len as usize } else { 0 });
        for _ in 0..len {
            buf.push(read(reader.by_ref())?);
        }
        Ok(buf)
    }

    fn read_component(reader: &mut impl io::Read) -> io::Result<Compound> {
        let mut map = IndexMap::default();
        loop {
            let tag = Self::read_tag(reader.by_ref())?;
            if tag == Tag::End {
                return Ok(Compound(map));
            }
            map.insert(
                Self::read_string(reader.by_ref())?,
                Self::read_value(reader.by_ref(), tag)?,
            );
        }
    }

    fn read_int_array(reader: &mut impl io::Read) -> io::Result<Vec<i32>> {
        let len = reader.read_i32::<BigEndian>()?;
        if len.is_negative() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "int array length is negative",
            ));
        }
        let mut buf = vec![0; len as usize];
        for _ in 0..len {
            buf.push(reader.read_i32::<BigEndian>()?);
        }
        Ok(buf)
    }

    fn read_long_array(reader: &mut impl io::Read) -> io::Result<Vec<i64>> {
        let len = reader.read_i32::<BigEndian>()?;
        if len.is_negative() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "long array length is negative",
            ));
        }
        let mut buf = vec![0; len as usize];
        for _ in 0..len {
            buf.push(reader.read_i64::<BigEndian>()?);
        }
        Ok(buf)
    }

    fn write_tag(writer: &mut impl io::Write, tag: Tag) -> io::Result<()> {
        writer.write_u8(tag as u8)
    }

    fn write_value(writer: &mut impl io::Write, value: &Value) -> io::Result<()> {
        match value {
            Value::Byte(v) => Self::write_byte(writer.by_ref(), *v),
            Value::Short(v) => Self::write_short(writer.by_ref(), *v),
            Value::Int(v) => Self::write_int(writer.by_ref(), *v),
            Value::Long(v) => Self::write_long(writer.by_ref(), *v),
            Value::Float(v) => Self::write_float(writer.by_ref(), *v),
            Value::Double(v) => Self::write_double(writer.by_ref(), *v),
            Value::ByteArray(v) => Self::write_byte_array(writer.by_ref(), v),
            Value::String(v) => Self::write_string(writer.by_ref(), v),
            Value::List(v) => Self::write_list(writer.by_ref(), v),
            Value::Compound(v) => Self::write_component(writer.by_ref(), v),
            Value::IntArray(v) => Self::write_int_array(writer.by_ref(), v),
            Value::LongArray(v) => Self::write_long_array(writer.by_ref(), v),
        }
    }

    fn write_byte(writer: &mut impl io::Write, value: i8) -> io::Result<()> {
        writer.write_i8(value)
    }

    fn write_short(writer: &mut impl io::Write, value: i16) -> io::Result<()> {
        writer.write_i16::<BigEndian>(value)
    }

    fn write_int(writer: &mut impl io::Write, value: i32) -> io::Result<()> {
        writer.write_i32::<BigEndian>(value)
    }

    fn write_long(writer: &mut impl io::Write, value: i64) -> io::Result<()> {
        writer.write_i64::<BigEndian>(value)
    }

    fn write_float(writer: &mut impl io::Write, value: f32) -> io::Result<()> {
        writer.write_f32::<BigEndian>(value)
    }

    fn write_double(writer: &mut impl io::Write, value: f64) -> io::Result<()> {
        writer.write_f64::<BigEndian>(value)
    }

    fn write_byte_array(writer: &mut impl io::Write, value: &[i8]) -> io::Result<()> {
        let len = i32::try_from(value.len()).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("byte array too long: {}", value.len()),
            )
        })?;
        writer.write_i32::<BigEndian>(len)?;
        for &b in value {
            writer.write_i8(b)?;
        }
        Ok(())
    }

    fn write_string(writer: &mut impl io::Write, value: &str) -> io::Result<()> {
        let len = u16::try_from(value.len()).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("string too long: {}", value.len()),
            )
        })?;
        writer.write_u16::<BigEndian>(len)?;
        writer.write_all(value.as_bytes())?;
        Ok(())
    }

    fn write_list(writer: &mut impl io::Write, list: &List) -> io::Result<()> {
        match list {
            List::End => {
                Self::write_tag(writer.by_ref(), Tag::End)?;
                writer.write_i32::<BigEndian>(0)
            }
            List::Byte(vec) => {
                Self::write_list_of(writer.by_ref(), vec, |w, v| Self::write_byte(w, *v))
            }
            List::Short(vec) => {
                Self::write_list_of(writer.by_ref(), vec, |w, v| Self::write_short(w, *v))
            }
            List::Int(vec) => {
                Self::write_list_of(writer.by_ref(), vec, |w, v| Self::write_int(w, *v))
            }
            List::Long(vec) => {
                Self::write_list_of(writer.by_ref(), vec, |w, v| Self::write_long(w, *v))
            }
            List::Float(vec) => {
                Self::write_list_of(writer.by_ref(), vec, |w, v| Self::write_float(w, *v))
            }
            List::Double(vec) => {
                Self::write_list_of(writer.by_ref(), vec, |w, v| Self::write_double(w, *v))
            }
            List::ByteArray(vec) => {
                Self::write_list_of(writer.by_ref(), vec, |w, v| Self::write_byte_array(w, v))
            }
            List::String(vec) => {
                Self::write_list_of(writer.by_ref(), vec, |w, v| Self::write_string(w, v))
            }
            List::List(vec) => {
                Self::write_list_of(writer.by_ref(), vec, |w, v| Self::write_list(w, v))
            }
            List::Compound(vec) => {
                Self::write_list_of(writer.by_ref(), vec, |w, v| Self::write_component(w, v))
            }
            List::IntArray(vec) => {
                Self::write_list_of(writer.by_ref(), vec, |w, v| Self::write_int_array(w, v))
            }
            List::LongArray(vec) => {
                Self::write_list_of(writer.by_ref(), vec, |w, v| Self::write_long_array(w, v))
            }
        }
    }

    fn write_list_of<T, W: io::Write>(
        writer: &mut W,
        list: &[T],
        mut write: impl FnMut(&mut W, &T) -> io::Result<()>,
    ) -> io::Result<()> {
        let len = i32::try_from(list.len()).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("list too long: {}", list.len()),
            )
        })?;
        writer.write_i32::<BigEndian>(len)?;
        for v in list {
            write(writer, v)?;
        }
        Ok(())
    }

    fn write_component(writer: &mut impl io::Write, compound: &Compound) -> io::Result<()> {
        for (name, value) in &compound.0 {
            Self::write_tag(writer.by_ref(), value.tag())?;
            Self::write_string(writer.by_ref(), name)?;
            Self::write_value(writer.by_ref(), value)?;
        }
        Self::write_tag(writer, Tag::End)
    }

    fn write_int_array(writer: &mut impl io::Write, array: &[i32]) -> io::Result<()> {
        let len = i32::try_from(array.len()).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("int array too long: {}", array.len()),
            )
        })?;
        writer.write_i32::<BigEndian>(len)?;
        for &v in array {
            writer.write_i32::<BigEndian>(v)?;
        }
        Ok(())
    }

    fn write_long_array(writer: &mut impl io::Write, array: &[i64]) -> io::Result<()> {
        let len = i32::try_from(array.len()).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("long array too long: {}", array.len()),
            )
        })?;
        writer.write_i32::<BigEndian>(len)?;
        for &v in array {
            writer.write_i64::<BigEndian>(v)?;
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Compound(pub IndexMap<String, Value>);

impl Compound {
    /// Same as [`Component::read`] but excludes the root name.
    pub fn read_network(mut reader: impl io::Read) -> io::Result<Self> {
        let root = Value::read_tag(&mut reader)?;

        if root != Tag::Compound {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("expected compound tag, got {root:?}"),
            ));
        }

        let root = Value::read_component(&mut reader)?;

        Ok(root)
    }

    pub fn read(mut reader: impl io::Read) -> io::Result<(String, Self)> {
        let root = Value::read_tag(&mut reader)?;

        if root != Tag::Compound {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("expected compound tag, got {root:?}"),
            ));
        }

        let root_name = Value::read_string(&mut reader)?;
        let root = Value::read_component(&mut reader)?;

        Ok((root_name, root))
    }

    pub fn write_network(&self, mut writer: impl io::Write) -> io::Result<()> {
        Value::write_tag(&mut writer, Tag::Compound)?;
        Value::write_component(&mut writer, self)
    }

    pub fn write(&self, mut writer: impl io::Write, name: &str) -> io::Result<()> {
        Value::write_tag(&mut writer, Tag::Compound)?;
        Value::write_string(&mut writer, name)?;
        Value::write_component(&mut writer, self)
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub enum List {
    #[default]
    End,
    Byte(Vec<i8>),
    Short(Vec<i16>),
    Int(Vec<i32>),
    Long(Vec<i64>),
    Float(Vec<f32>),
    Double(Vec<f64>),
    ByteArray(Vec<Vec<i8>>),
    String(Vec<String>),
    List(Vec<List>),
    Compound(Vec<Compound>),
    IntArray(Vec<Vec<i32>>),
    LongArray(Vec<Vec<i64>>),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tag {
    End = 0,
    Byte = 1,
    Short = 2,
    Int = 3,
    Long = 4,
    Float = 5,
    Double = 6,
    ByteArray = 7,
    String = 8,
    List = 9,
    Compound = 10,
    IntArray = 11,
    LongArray = 12,
}
