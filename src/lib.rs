//! This is a companion crate for `iref` providing two macros to build `'static`
//! IRIs and IRI references at compile time.
//!
//! ## Basic usage
//!
//! Use the `iri!` macro to build IRI statically, and the `iref!` macro to build
//! IRI references statically.
//!
//! ```rust
//! extern crate iref;
//! #[macro_use]
//! extern crate static_iref;
//!
//! use iref::{Iri, IriRef};
//!
//! const IRI: Iri<'static> = iri!("https://www.rust-lang.org/foo/bar#frag");
//! const IREF: IriRef<'static> = iref!("/foo/bar#frag");
//! ```

extern crate iref;
extern crate proc_macro;

use iref::{Iri, IriRef};
use proc_macro::{TokenStream, TokenTree};
use std::fmt;

struct Optional<T>(Option<T>);

impl<T: fmt::Display> fmt::Display for Optional<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match &self.0 {
			Some(t) => write!(f, "Some({})", t),
			None => write!(f, "None"),
		}
	}
}

fn string_literal(tokens: TokenStream) -> Result<String, &'static str> {
	if let Some(token) = tokens.into_iter().next() {
		if let TokenTree::Literal(lit) = token {
			let str = lit.to_string();

			if str.len() >= 2 {
				let mut buffer = String::with_capacity(str.len() - 2);
				for (i, c) in str.chars().enumerate() {
					if i == 0 || i == str.len() - 1 {
						if c != '"' {
							return Err("expected string literal");
						}
					} else {
						buffer.push(c)
					}
				}

				Ok(buffer)
			} else {
				Err("expected string literal")
			}
		} else {
			Err("expected string literal")
		}
	} else {
		Err("expected one string parameter")
	}
}

fn stringify_authority_parsing_data(p: Option<iref::parsing::ParsedAuthority>) -> String {
	match p {
		Some(p) => {
			format!(
				"Some(::iref::parsing::ParsedAuthority {{
	userinfo_len: {},
	host_len: {},
	port_len: {}
}})
",
				Optional(p.userinfo_len),
				p.host_len,
				Optional(p.port_len)
			)
		}
		None => "None".to_string(),
	}
}

fn stringify_parsing_data(p: iref::parsing::ParsedIriRef) -> String {
	format!(
		"::iref::parsing::ParsedIriRef {{
	scheme_len: {},
	authority: {},
	path_len: {},
	query_len: {},
	fragment_len: {}
}}",
		Optional(p.scheme_len),
		stringify_authority_parsing_data(p.authority),
		p.path_len,
		Optional(p.query_len),
		Optional(p.fragment_len)
	)
}

/// Build an IRI reference with a `'static` lifetime at compile time.
///
/// This macro expects a single string literal token representing the IRI reference.
#[proc_macro]
pub fn iref(tokens: TokenStream) -> TokenStream {
	match string_literal(tokens) {
		Ok(str) => {
			if let Ok(iri_ref) = IriRef::new(str.as_str()) {
				let p = stringify_parsing_data(iri_ref.parsing_data());
				format!("unsafe{{::iref::IriRef::from_raw(b\"{}\", {})}}", str, p)
					.parse()
					.unwrap()
			} else {
				produce_error("invalid IRI reference")
			}
		}
		Err(msg) => produce_error(msg),
	}
}

/// Build an IRI with a `'static` lifetime at compile time.
///
/// This macro expects a single string literal token representing the IRI.
#[proc_macro]
pub fn iri(tokens: TokenStream) -> TokenStream {
	match string_literal(tokens) {
		Ok(str) => {
			if let Ok(iri) = Iri::new(str.as_str()) {
				let p = stringify_parsing_data(iri.as_iri_ref().parsing_data());
				format!(
					"::iref::Iri::from_iri_ref(unsafe{{::iref::IriRef::from_raw(b\"{}\", {})}})",
					str, p
				)
				.parse()
				.unwrap()
			} else {
				produce_error("invalid IRI")
			}
		}
		Err(msg) => produce_error(msg),
	}
}

fn produce_error(msg: &str) -> TokenStream {
	format!("compile_error!(\"{}\")", msg).parse().unwrap()
}
