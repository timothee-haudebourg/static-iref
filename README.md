# Build IRI and IRI references at compile time

<table><tr>
	<td><a href="https://docs.rs/static-iref">Documentation</a></td>
	<td><a href="https://crates.io/crates/static-iref">Crate informations</a></td>
	<td><a href="https://github.com/timothee-haudebourg/static-iref">Repository</a></td>
</tr></table>

This is a companion crate for `iref` providing two macros to build `'static`
IRIs and IRI references at compile time.

## Basic usage

Use the `iri!` macro to build IRI statically, and the `iref!` macro to build
IRI references statically.

```rust
extern crate iref;
#[macro_use]
extern crate static_iref;

use iref::{Iri, IriRef};

const IRI: Iri<'static> = iri!("https://www.rust-lang.org/foo/bar#frag");
const IREF: IriRef<'static> = iref!("/foo/bar#frag");
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
