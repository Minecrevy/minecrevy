use crate::{McRead, McWrite};

macro_rules! impl_tuples {
    ($(($($type:ident $value:ident $arg:ident),+),)+) => {
        $(
            impl<$($type: McRead),+> McRead for ($($type),+,) {
                type Args = ($($type::Args,)+);

                fn read(mut reader: impl std::io::Read, args: Self::Args) -> std::io::Result<Self> {
                    let ($($arg,)+) = args;
                    Ok(($($type::read(&mut reader, $arg)?,)+))
                }
            }

            impl<$($type: McWrite),+> McWrite for ($($type),+,) {
                type Args = ($($type::Args,)+);

                fn write(&self, mut writer: impl std::io::Write, args: Self::Args) -> std::io::Result<()> {
                    let ($($arg,)+) = args;
                    let ($($value,)+) = self;
                    $(
                        $value.write(&mut writer, $arg)?;
                    )+
                    Ok(())
                }
            }
        )+
    };
}

impl_tuples! {
    (A a aa),
    (A a aa, B b bb),
    (A a aa, B b bb, C c cc),
    (A a aa, B b bb, C c cc, D d dd),
    (A a aa, B b bb, C c cc, D d dd, E e ee),
    (A a aa, B b bb, C c cc, D d dd, E e ee, F f ff),
    (A a aa, B b bb, C c cc, D d dd, E e ee, F f ff, G g gg),
    (A a aa, B b bb, C c cc, D d dd, E e ee, F f ff, G g gg, H h hh),
    (A a aa, B b bb, C c cc, D d dd, E e ee, F f ff, G g gg, H h hh, I i ii),
    (A a aa, B b bb, C c cc, D d dd, E e ee, F f ff, G g gg, H h hh, I i ii, J j jj),
    (A a aa, B b bb, C c cc, D d dd, E e ee, F f ff, G g gg, H h hh, I i ii, J j jj, K k kk),
    (A a aa, B b bb, C c cc, D d dd, E e ee, F f ff, G g gg, H h hh, I i ii, J j jj, K k kk, L l ll),
}
