# Schema Design

A `Request` provides a `Schema` via `schema()`:

```rust
pub trait Request {
    fn schema(&self) -> impl Schema;
}
```

- **Not a reference**: This allows the return value to be zero-sized (ZST) and evaluated/optimized away at compile time.
- **Push-based filtering**: The `Data` implementation queries the request and pushes values. The request checks `self.schema().accepts_value(&value)` before executing the visitor to avoid overhead.

## Type representation

Type sets and types are represented by the `Type` and `TypeSet` traits:

* `TypeOf<T>` wraps `PhantomData<T>` to represent type `T` at compile time as a zero-sized type.
* `SimpleTypeSet` provides runtime collections of `TypeId`s and name strings for type matching.