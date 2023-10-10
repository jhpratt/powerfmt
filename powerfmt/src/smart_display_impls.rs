//! Implementation of [`SmartDisplay`] for various types.

#[cfg(feature = "alloc")]
use alloc::borrow::{Cow, ToOwned};
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::rc::Rc;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::sync::Arc;
use core::cell::{Ref, RefMut};
use core::cmp::min;
use core::convert::Infallible;
use core::fmt::{self, Display, Formatter};
use core::num::Wrapping;
use core::pin::Pin;

use crate::smart_display::{FormatterOptions, Metadata, SmartDisplay};

impl SmartDisplay for Infallible {
    type Metadata = Self;

    #[inline]
    fn metadata(&self, _: FormatterOptions) -> Metadata<Self> {
        match *self {}
    }

    #[inline]
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        match *self {}
    }
}

impl SmartDisplay for bool {
    type Metadata = ();

    #[inline]
    fn metadata(&self, _: FormatterOptions) -> Metadata<()> {
        Metadata::new(if *self { 4 } else { 5 }, ())
    }

    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl SmartDisplay for str {
    type Metadata = ();

    #[inline]
    fn metadata(&self, f: FormatterOptions) -> Metadata<()> {
        Metadata::new(
            match f.precision() {
                Some(max_len) => min(self.len(), max_len),
                None => self.len(),
            },
            (),
        )
    }

    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

#[cfg(feature = "alloc")]
impl SmartDisplay for String {
    type Metadata = ();

    #[inline]
    fn metadata(&self, f: FormatterOptions) -> Metadata<()> {
        (**self).metadata(f)
    }

    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

#[cfg(feature = "alloc")]
impl<'a, B, O> SmartDisplay for Cow<'a, B>
where
    B: SmartDisplay + ToOwned<Owned = O> + ?Sized,
    O: SmartDisplay<Metadata = B::Metadata> + 'a,
{
    type Metadata = B::Metadata;

    fn metadata(&self, f: FormatterOptions) -> Metadata<'_, Self::Metadata> {
        match *self {
            Cow::Borrowed(ref b) => b.metadata(f),
            Cow::Owned(ref o) => o.metadata(f),
        }
    }

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl<T> SmartDisplay for Pin<&T>
where
    T: SmartDisplay + ?Sized,
{
    type Metadata = T::Metadata;

    fn metadata(&self, f: FormatterOptions) -> Metadata<Self::Metadata> {
        self.get_ref().metadata(f)
    }

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        SmartDisplay::fmt(self.get_ref(), f)
    }
}

impl<T> SmartDisplay for &T
where
    T: SmartDisplay + ?Sized,
{
    type Metadata = T::Metadata;

    fn metadata(&self, f: FormatterOptions) -> Metadata<Self::Metadata> {
        (**self).metadata(f)
    }

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        SmartDisplay::fmt(*self, f)
    }
}

impl<T> SmartDisplay for &mut T
where
    T: SmartDisplay + ?Sized,
{
    type Metadata = T::Metadata;

    fn metadata(&self, f: FormatterOptions) -> Metadata<Self::Metadata> {
        (**self).metadata(f)
    }

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        SmartDisplay::fmt(*self, f)
    }
}

impl<T> SmartDisplay for Ref<'_, T>
where
    T: SmartDisplay + ?Sized,
{
    type Metadata = T::Metadata;

    fn metadata(&self, f: FormatterOptions) -> Metadata<Self::Metadata> {
        (**self).metadata(f)
    }

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        SmartDisplay::fmt(&**self, f)
    }
}

impl<T> SmartDisplay for RefMut<'_, T>
where
    T: SmartDisplay + ?Sized,
{
    type Metadata = T::Metadata;

    fn metadata(&self, f: FormatterOptions) -> Metadata<Self::Metadata> {
        (**self).metadata(f)
    }

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        SmartDisplay::fmt(&**self, f)
    }
}

impl<T> SmartDisplay for Wrapping<T>
where
    T: SmartDisplay,
{
    type Metadata = T::Metadata;

    fn metadata(&self, f: FormatterOptions) -> Metadata<Self::Metadata> {
        self.0.metadata(f)
    }

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        SmartDisplay::fmt(&self.0, f)
    }
}

#[cfg(feature = "alloc")]
impl<T> SmartDisplay for Rc<T>
where
    T: SmartDisplay + ?Sized,
{
    type Metadata = T::Metadata;

    fn metadata(&self, f: FormatterOptions) -> Metadata<Self::Metadata> {
        (**self).metadata(f)
    }

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        SmartDisplay::fmt(&**self, f)
    }
}

#[cfg(feature = "alloc")]
impl<T> SmartDisplay for Arc<T>
where
    T: SmartDisplay + ?Sized,
{
    type Metadata = T::Metadata;

    fn metadata(&self, f: FormatterOptions) -> Metadata<Self::Metadata> {
        (**self).metadata(f)
    }

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        SmartDisplay::fmt(&**self, f)
    }
}

#[cfg(feature = "alloc")]
impl<T> SmartDisplay for Box<T>
where
    T: SmartDisplay + ?Sized,
{
    type Metadata = T::Metadata;

    fn metadata(&self, f: FormatterOptions) -> Metadata<Self::Metadata> {
        (**self).metadata(f)
    }

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        SmartDisplay::fmt(&**self, f)
    }
}

/// Implement [`SmartDisplay`] for unsigned integers.
macro_rules! impl_uint {
    ($($t:ty)*) => {$(
        impl SmartDisplay for $t {
            type Metadata = ();

            fn metadata(&self, _: FormatterOptions) -> Metadata<'_, Self::Metadata> {
                let width = self.checked_ilog10().map_or(1, |n| n as usize + 1);
                Metadata::new(width, ())
            }

            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                Display::fmt(self, f)
            }
        }
    )*};
}

impl_uint![u8 u16 u32 u64 u128 usize];

/// Implement [`SmartDisplay`] for signed integers.
macro_rules! impl_int {
    ($($t:ty)*) => {$(
        impl SmartDisplay for $t {
            type Metadata = ();

            fn metadata(&self, _: FormatterOptions) -> Metadata<'_, Self::Metadata> {
                let mut width = if *self < 0 { 1 } else { 0 };
                width += self.unsigned_abs().checked_ilog10().map_or(1, |n| n as usize + 1);
                Metadata::new(width, ())
            }

            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                Display::fmt(self, f)
            }
        }
    )*};
}

impl_int![i8 i16 i32 i64 i128 isize];
