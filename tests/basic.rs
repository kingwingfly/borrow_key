use borrow_key::BorrowKey;

#[derive(Debug, BorrowKey)]
struct Foo {
    #[key(str)]
    key: String,
    value: u8,
}

#[test]
fn test() {
    use std::collections::HashSet;

    let foo0 = Foo {
        key: "1".to_string(),
        value: 0,
    };
    let foo1 = Foo {
        key: "1".to_string(),
        value: 1,
    };
    let foo2 = Foo {
        key: "2".to_string(),
        value: 2,
    };
    assert_eq!(foo0, foo1);

    let set = HashSet::from([foo1, foo2]);

    assert_eq!(set.get("1").unwrap().value, 1);
    assert_eq!(set.get("2").unwrap().value, 2);
}
