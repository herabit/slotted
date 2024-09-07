mod sealed {
    pub trait Sealed {}
}

/// Marker trait for types that denote some kind of "accessibility" for pointers and references.
pub trait Access: sealed::Sealed {
    type Refer<'a, T: ?Sized + 'a>;
    type Pointer<T: ?Sized>;

    type Inverse: Access<Inverse = Self>;

    // fn reborrow<'a, 'b, T>(orig: Self::R)

    fn map_ref<'a, T1, T2, F>(orig: Self::Refer<'a, T1>, f: F) -> Self::Refer<'a, T2>
    where
        T1: ?Sized + 'a,
        T2: ?Sized + 'a,
        F: FnOnce(Self::Refer<'a, T1>) -> Self::Refer<'a, T2>;
}

/// Marker type for immutable references and pointers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Const;

impl sealed::Sealed for Const {}
impl Access for Const {
    type Refer<'a, T: ?Sized + 'a> = &'a T;
    type Pointer<T: ?Sized> = *const T;

    type Inverse = Mut;

    #[inline]
    fn map_ref<'a, T1, T2, F>(orig: Self::Refer<'a, T1>, f: F) -> Self::Refer<'a, T2>
    where
        T1: ?Sized + 'a,
        T2: ?Sized + 'a,
        F: FnOnce(Self::Refer<'a, T1>) -> Self::Refer<'a, T2>,
    {
        f(orig)
    }
}

/// Marker type for mutable references and pointers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Mut;

impl sealed::Sealed for Mut {}
impl Access for Mut {
    type Refer<'a, T: ?Sized + 'a> = &'a mut T;
    type Pointer<T: ?Sized> = *mut T;

    type Inverse = Const;

    #[inline]
    fn map_ref<'a, T1, T2, F>(orig: Self::Refer<'a, T1>, f: F) -> Self::Refer<'a, T2>
    where
        T1: ?Sized + 'a,
        T2: ?Sized + 'a,
        F: FnOnce(Self::Refer<'a, T1>) -> Self::Refer<'a, T2>,
    {
        f(orig)
    }
}
