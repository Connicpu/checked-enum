use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

pub trait CheckedEnum: Sized + 'static {
    type Storage: Copy + Clone + 'static;

    fn try_from_storage(val: Self::Storage) -> Option<Self>;
    fn to_storage(self) -> Self::Storage;
}

pub struct UncheckedEnum<T>
where
    T: CheckedEnum,
{
    pub value: T::Storage,
}

impl<T> Copy for UncheckedEnum<T> where T: CheckedEnum {}
impl<T> Clone for UncheckedEnum<T>
where
    T: CheckedEnum,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> fmt::Debug for UncheckedEnum<T>
where
    T: CheckedEnum,
    T: fmt::Debug,
    T::Storage: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("UncheckedEnum")
            .field("enum", &self.as_enum())
            .field("value", &self.value)
            .finish()
    }
}

impl<T> PartialEq for UncheckedEnum<T>
where
    T: CheckedEnum,
    T::Storage: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }

    fn ne(&self, other: &Self) -> bool {
        self.value != other.value
    }
}

impl<T> PartialEq<T> for UncheckedEnum<T>
where
    T: CheckedEnum,
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        T::try_from_storage(self.value).as_ref() == Some(other)
    }
}

impl<T> PartialEq<Option<T>> for UncheckedEnum<T>
where
    T: CheckedEnum,
    T: PartialEq,
{
    fn eq(&self, other: &Option<T>) -> bool {
        T::try_from_storage(self.value) == *other
    }
}

impl<T> Eq for UncheckedEnum<T>
where
    T: CheckedEnum,
    T::Storage: Eq,
{
}

impl<T> PartialOrd for UncheckedEnum<T>
where
    T: CheckedEnum,
    T::Storage: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T> Ord for UncheckedEnum<T>
where
    T: CheckedEnum,
    T::Storage: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T> Hash for UncheckedEnum<T>
where
    T: CheckedEnum,
    T::Storage: Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.value.hash(state)
    }
}

impl<T> UncheckedEnum<T>
where
    T: CheckedEnum,
{
    #[inline]
    pub fn new(value: T::Storage) -> Self {
        UncheckedEnum { value }
    }

    #[inline]
    pub fn as_enum(self) -> Option<T> {
        T::try_from_storage(self.value)
    }

    #[inline]
    pub unsafe fn as_enum_unchecked(&self) -> T {
        ::std::mem::transmute_copy(self)
    }
}

impl<T> From<T> for UncheckedEnum<T>
where
    T: CheckedEnum,
{
    fn from(val: T) -> UncheckedEnum<T> {
        UncheckedEnum::new(T::to_storage(val))
    }
}

macro_rules! imp_from_prim {
    ($prim:ident) => {
        impl<T> From< $prim > for UncheckedEnum<T>
        where
            T: CheckedEnum<Storage = $prim>,
        {
            #[inline]
            fn from(value: T::Storage) -> Self {
                UncheckedEnum::new(value)
            }
        }
    };
    ($($prim:ident)*) => {
        $(imp_from_prim!($prim);)*
    }
}

imp_from_prim!(i8 u8 i16 u16 i32 u32 i64 u64 isize usize);

#[macro_export]
macro_rules! checked_enum {
    ($ety:ident($repr:ty) => { $from:ident, $to:ident }) => {
        impl $crate::CheckedEnum for $ety {
            type Storage = $repr;

            #[inline]
            fn try_from_storage(val: Self::Storage) -> Option<Self> {
                $ety::$from(val)
            }

            #[inline]
            fn to_storage(self) -> Self::Storage {
                $ety::$to(self)
            }
        }
    };
}
