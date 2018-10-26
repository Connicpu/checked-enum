#[macro_use]
extern crate checked_enum;

use checked_enum::UncheckedEnum;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Foo {
    A = 0,
    B = 1,
}

checked_enum!(Foo(u32) => { from_u32, to_u32 });

impl Foo {
    pub fn from_u32(val: u32) -> Option<Foo> {
        match val {
            0 => Some(Foo::A),
            1 => Some(Foo::B),
            _ => None,
        }
    }

    pub fn to_u32(self) -> u32 {
        self as u32
    }
}

#[test]
fn basics() {
    assert_eq!(UncheckedEnum::from(Foo::A), UncheckedEnum::new(0));
    assert_eq!(UncheckedEnum::from(Foo::B), UncheckedEnum::new(1));
    assert_eq!(UncheckedEnum::new(0), Foo::A);
    assert_eq!(UncheckedEnum::new(1), Foo::B);
    assert_eq!(UncheckedEnum::new(0), Some(Foo::A));
    assert_eq!(UncheckedEnum::new(1), Some(Foo::B));
    assert_eq!(UncheckedEnum::new(2), None::<Foo>);
}
