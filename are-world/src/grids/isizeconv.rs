//! by *StarvinCulex @2021/12/03*

use std::cmp::Ord;
use std::marker::PhantomData;

use duplicate::duplicate;

use super::{Coord, Interval};

pub struct TypeTest<T, U>(PhantomData<T>, PhantomData<U>);

pub auto trait TypeNe {}

impl<T> !TypeNe for TypeTest<T, T> {}

duplicate! {
    [
        WrapperType   Decl(T)          convert(t, into);
        [ Coord    ]  [ T      ]       [ Coord(t.0.into, t.1.into)             ];
        [ Interval ]  [ T: Ord ]       [ Interval::new(t.from.into, t.to.into) ];
    ]
impl<Decl([T]), Decl([U])> From<WrapperType<T>> for WrapperType<U>
where
    TypeTest<T, U>: TypeNe,
    T: Into<U>,
{
    #[inline]
    fn from(t: WrapperType<T>) -> Self {
        convert([t], [into()])
    }
}
impl<Decl([T]), Decl([U])> TryFrom<WrapperType<T>> for WrapperType<U>
where
    TypeTest<T, U>: TypeNe,
    T: TryInto<U>,
{
    type Error = <T as TryInto<U>>::Error;
    #[inline]
    fn try_from(t: WrapperType<T>) -> Result<Self, Self::Error> {
        Ok(convert([t], [try_into()?]))
    }
}
}

#[cfg(test)]
#[test]
fn conv_test() {
    let coord1: Coord<i32> = 1.into();
    let coord2: Coord<i8> = coord1.try_into().unwrap();
    let coord3: Coord<i16> = coord2.into();

    let interval1: Interval<i32> = Interval::new(1, 2);
    let interval2: Interval<i8> = interval1.try_into().unwrap();
    let interval3: Interval<i16> = interval2.into();

    let coord_interval1: Coord<Interval<i32>> = Interval::new(1, 2).into();
    let coord_interval2: Coord<Interval<i8>> = coord_interval1.try_into().unwrap();
    let coord_interval3: Coord<Interval<i16>> = coord_interval2.into();
}
