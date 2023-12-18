// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2023 Taylan Gökkaya

use crate::*;

#[test]
fn test_consistency() {
	let tests = [
		"<p>foo</p>",
		"<failme></<failme>",
		"<p><ul><li>1</li></p></ul>",
		"",
	];

	let opts = FormatOptions::default();

	for s in tests {
		let str_1 = format(s, false, &opts);
		let mut str_2 = String::from("asdf");
		let str_2 = format_to(s, &mut str_2, false, &opts).map(move |_| str_2);

		let vec_1 = format_bytes(s, false, &opts);
		let mut vec_2 = Vec::from(b"foo");
		let vec_2 = format_bytes_to(s, &mut vec_2, false, &opts).map(move |_| vec_2);

		assert_eq!(str_1, str_2);
		assert_eq!(vec_1, vec_2);
		assert_eq!(str_1.as_ref().map(|s| s.as_bytes()), vec_1.as_deref());
	}
}
