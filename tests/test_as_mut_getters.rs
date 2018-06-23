#[macro_use]
extern crate enum_methods;

#[test]
fn test_as_getters() {
    #[derive(EnumAsMutGetters, Debug)]
    enum MyEnum {
        Foo(i64),
        Bar(bool),
        Baz(String),
        Tup(i32, String, Vec<bool>),
    }

    let mut foo = MyEnum::Foo(42);
    let mut bar = MyEnum::Bar(false);
    let mut baz = MyEnum::Baz("hurry boy, it's waiting there for you".to_string());
    let mut tup = MyEnum::Tup(42, String::from("Hello, Tuple, my old friend!"), vec![true, false, true]);
    assert_eq!(*foo.as_foo_mut(), 42);
    assert_eq!(*bar.as_bar_mut(), false);
    assert_eq!(baz.as_baz_mut(), "hurry boy, it's waiting there for you");
    assert_eq!(tup.as_tup_mut(), (&mut 42, &mut String::from("Hello, Tuple, my old friend!"), &mut vec![true, false, true]));

    *foo.as_foo_mut() = 84;
    assert_eq!(*foo.as_foo_mut(), 84);
}

#[test]
fn test_as_getter_names() {
    #[derive(EnumAsMutGetters, Debug)]
    enum MyEnum {
        FooBar(bool),
        BarBaz(String),
    }

    let mut first = MyEnum::FooBar(true);
    let mut second = MyEnum::BarBaz(
        "there's nothing that a hundred men or more could ever do".to_string(),
    );
    assert_eq!(*first.as_foo_bar_mut(), true);
    assert_eq!(
        second.as_bar_baz_mut(),
        "there's nothing that a hundred men or more could ever do"
    );
}

#[test]
fn test_getter_structs() {
    #[derive(EnumAsMutGetters, Debug)]
    enum MyEnum {
        FooBar(bool),
        BarBaz(String),
        SomeStruct { foo: i32 }, // should be skipped
    }

    impl MyEnum {
        pub fn as_some_struct_mut(&mut self) -> &mut i32 {
            if let &mut MyEnum::SomeStruct { ref mut foo } = self {
                foo
            } else {
                unreachable!()
            }
        }
    }

    let mut first = MyEnum::FooBar(true);
    let mut second = MyEnum::BarBaz(
        "there's nothing that a hundred men or more could ever do".to_string(),
    );
    let mut third = MyEnum::SomeStruct { foo: 42 };
    assert_eq!(*first.as_foo_bar_mut(), true);
    assert_eq!(
        second.as_bar_baz_mut(),
        "there's nothing that a hundred men or more could ever do"
    );
    assert_eq!(*third.as_some_struct_mut(), 42);
}
