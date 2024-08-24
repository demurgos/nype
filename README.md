# Strype - Strong Type

Strype is a Rust library. It provices _declarative_ macros to define strong types. A strong type places stricter
requirements on values and semantic meaning. For example, instead of `String` you may use a type `Username` which
ensures that the inner value respects your username requirements (such as legal characters, a max length or some pattern).

Strype provides integration with libraries such as `serde` or `sqlx` to allow ergonomic usage.

Strype is not related to the company Stripe.

# License


