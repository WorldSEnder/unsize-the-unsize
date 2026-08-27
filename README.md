# Unsize-the-unsize

Rust does not allow unsizing types that are not sized.
This allows this for a limited set of types.

```rust
#[test]
fn coerce_str_any() {
    let mut host = Host::new();
    let as_any: &dyn Any = host.coerce("foobar");
    assert_eq!(as_any.type_id(), TypeId::of::<str>());
}
```

## Is this safe?

Lol.

## Ethics Approval

Ethical approval was not sought for the present repository for mental health reasons.

## Funding

None. [^1]

[^1]: Shout out to my github sponsor @Madoshakalaka. No affiliation or approval of his to this project is implied.
