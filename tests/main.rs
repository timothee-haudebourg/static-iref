#![feature(proc_macro_hygiene)]

extern crate iref;
#[macro_use]
extern crate static_iref;

use iref::{Iri, IriRef};

const IREF: IriRef<'static> = iref!("/foo/bar#frag");
const IRI: Iri<'static> = iri!("https://www.rust-lang.org/foo/bar#frag");

#[test]
fn iri_ref() {
	assert_eq!(IREF, IriRef::new("/foo/bar#frag").unwrap());
}

#[test]
fn iri() {
	assert_eq!(
		IRI,
		Iri::new("https://www.rust-lang.org/foo/bar#frag").unwrap()
	);
}
