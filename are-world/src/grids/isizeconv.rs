//! by *StarvinCulex @2021/12/03*

use super::{Coord, Interval};

macro_rules! try_into_impl {
    ($a:ident, $b:ident) => {
        impl std::convert::TryInto<Coord<$b>> for Coord<$a> {
            type Error = std::num::TryFromIntError;
            #[inline]
            fn try_into(self) -> Result<Coord<$b>, Self::Error> {
                Ok(Coord(self.0.try_into()?, self.1.try_into()?))
            }
        }
        impl std::convert::TryInto<Interval<$b>> for Interval<$a> {
            type Error = std::num::TryFromIntError;
            #[inline]
            fn try_into(self) -> Result<Interval<$b>, Self::Error> {
                Ok(Interval::new(self.from.try_into()?, self.to.try_into()?))
            }
        }
        impl std::convert::TryInto<Coord<Interval<$b>>> for Coord<Interval<$a>> {
            type Error = std::num::TryFromIntError;
            #[inline]
            fn try_into(self) -> Result<Coord<Interval<$b>>, Self::Error> {
                Ok(Coord(self.0.try_into()?, self.1.try_into()?))
            }
        }
    };
}

try_into_impl!(isize, usize);
try_into_impl!(usize, isize);

try_into_impl!(i32, isize);
try_into_impl!(isize, i32);
try_into_impl!(i32, usize);
try_into_impl!(usize, i32);

try_into_impl!(u32, isize);
try_into_impl!(u32, usize);
try_into_impl!(isize, u32);
try_into_impl!(usize, u32);

try_into_impl!(i64, isize);
try_into_impl!(isize, i64);
try_into_impl!(i64, usize);
try_into_impl!(usize, i64);

try_into_impl!(u64, isize);
try_into_impl!(isize, u64);
try_into_impl!(u64, usize);
try_into_impl!(usize, u64);
