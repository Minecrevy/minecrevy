/// Specifies a C-like enum that is encoded/decoded with the specified functions.
#[macro_export]
macro_rules! mcio_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $ty:ident as $read:path, $write:path {
            $($variant:ident = $num:expr,)+
        }
    ) => {
        $(#[$meta])*
        $vis enum $ty {
            $($variant = $num,)+
        }

        impl ::minecrevy_io_str::McRead for $ty {
            type Options = ();

            fn read<R: ::std::io::Read>(mut reader: R, _options: Self::Options) -> ::std::io::Result<Self> {
                match $read(&mut reader)? {
                    $($num => Ok(Self::$variant),)+
                    v => Err(::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!("invalid {}: expected any of {:?}, got {}", ::std::any::type_name::<$ty>(), [$($num,)+], v),
                    ))
                }
            }
        }

        impl ::minecrevy_io_str::McWrite for $ty {
            type Options = ();

            fn write<W: ::std::io::Write>(&self, mut writer: W, _options: Self::Options) -> ::std::io::Result<()> {
                match self {
                    $(Self::$variant => $write(&mut writer, $num)?,)+
                }
                Ok(())
            }
        }
    };
}

// TODO: proc macro?
/// Specifies a C-like enum that is encoded/decoded as a varint.
#[macro_export]
macro_rules! varint_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $ty:ident {
            $($variant:ident = $num:expr,)+
        }
    ) => {
        ::minecrevy_io_str::mcio_enum! {
            $(#[$meta])*
            $vis enum $ty as ::minecrevy_io_buf::ReadMinecraftExt::read_var_i32, ::minecrevy_io_buf::WriteMinecraftExt::write_var_i32 {
                $($variant = $num,)+
            }
        }
    };
}

/// Specifies a C-like enum that is encoded/decoded as a byte.
#[macro_export]
macro_rules! u8_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $ty:ident {
            $($variant:ident = $num:expr,)+
        }
    ) => {
        ::minecrevy_io_str::mcio_enum! {
            $(#[$meta])*
            #[repr(u8)]
            $vis enum $ty as ::minecrevy_io_buf::ReadMinecraftExt::read_u8, ::minecrevy_io_buf::WriteMinecraftExt::write_u8 {
                $($variant = $num,)+
            }
        }
    };
}
