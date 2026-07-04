# Schema System

Datalink's schema system is used by `Request`s to validate and filter incoming values and links during a query.

## Schema Trait

The primary trait is `Schema`, defined in [mod.rs](file:///home/satan/projects/datalink/src/schema/mod.rs):

```rust
pub trait Schema {
    fn accepts_value<T: core::any::Any + ?Sized>(&self, value: &T) -> bool;
    fn accepts_value_of<T: core::any::Any + ?Sized>(&self) -> bool;
    fn accepts_value_type<T: Type>(&self, typ: T) -> bool;
    fn accepted_value_types(&self) -> impl TypeSet;

    fn accepts_link<L: Link>(&self, link: &L) -> bool;
    fn accepts_link_type<L: Link>(&self) -> bool;
    fn accepted_link_types(&self) -> impl IntoIterator<Item = (impl TypeSet, impl TypeSet)>;
}
```

By defining these filter methods, the `Request` can tell `Data` implementors at query-time what kinds of values and links it is willing to process, allowing `Data` queries to avoid expensive allocations or processing for unaccepted data.

## Built-in Schemas

* **`TRUE`**: Accepts all values and links.
* **`FALSE`**: Rejects all values and links.
* **`SimpleSchema`**: A concrete schema defined by a `SimpleTypeSet` of accepted value types and a list of accepted key-target type pairs for links.
* **`TypeOf<T>`**: A schema that only accepts values of type `T`.

## Schema Composition & Builders

Schemas can be composed using logical operators. The `SchemaExt` trait provides builders:

* `.and(other)`: Returns an `And` schema (requires both schemas to accept).
* `.or(other)`: Returns an `Or` schema (requires at least one schema to accept).
* `.invert()`: Returns a `Not` schema (negates the acceptance of the inner schema).

For example:
```rust
let schema = TypeOf::<String>::new().or(TypeOf::<u32>::new());
```