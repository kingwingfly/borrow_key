In this crate, we aiming to implemente a proc-macro named `BorrowKey`.
This macro should allow a struct to be borrowed as a reference to one of its fields, called `key`, based on `core::borrow::Borrow`.
And it should also ensure that Eq, Ord and Hash are implemented correctly according to the documentation requirements below,
so that this struct can be used in `HashSet` correctly:

> Further, when providing implementations for additional traits,
> it needs to be considered whether they should behave
> identically to those of the underlying type as a consequence of acting as a representation of that underlying type.
> Generic code typically uses `Borrow<T>` when it relies on the identical behavior of these additional trait implementations.
> These traits will likely appear as additional trait bounds.
>
> In particular Eq, Ord and Hash must be equivalent for borrowed and owned values: x.borrow() == y.borrow() should give the same result as x == y.
>
> If generic code merely needs to work for all types that can provide a reference to related type T,
> it is often better to use `AsRef<T>` as more types can safely implement it.

```rust
use std::collections::HashSet;
use borrow_key::BorrowKey;

#[derive(Debug, BorrowKey)]
struct Foo {
    #[key(str)]
    key: String,
    value: u8,
}

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
```
where
```rust ignore
#[derive(Debug, BorrowKey)]
struct Foo {
    #[key(str)]
    key: String,
    value: u8,
}
```
will be expanded to
```rust ignore
struct Foo {
    key: String,
    value: u8,
}

impl Borrow<str> for Foo {
    fn borrow(&self) -> &str {
        self.key.as_str()
    }
}

impl Hash for Foo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl PartialEq for Foo {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for Foo {}

impl PartialOrd for Foo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Foo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

```

This is useful when we need HashMap<&str, Foo>, and &str key ref to foo.key. This self-referential struct can be avoided by `HashSet` with special Borrow and Hash implemented.

For more information about why this is needed to be used in HashSet, see <https://github.com/kingwingfly/corust-hackathon/blob/dev/hashmap_but_key_ref_to_value/src/lib.rs>
