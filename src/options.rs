// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2023 Taylan Gökkaya

use tidy_sys::*;

macro_rules! set {
	[$doc:expr, $func:ident, $($id:ident = $val:expr),* $(,)?] => {{
		$(
			$func($doc, $id, $val);
		)*
	}};
}

/// Formatting options.
#[derive(Debug, Hash, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct FormatOptions {
	/// Options that control line indentation.
	pub indent: Indent,
	/// The line ending to use. The default is [`LineEnding::Lf`].
	pub line_ending: LineEnding,
	/// The maximum line width. Lines exceeding this value will be wrapped, if
	/// possible.
	///
	/// A value of `0` disables line wrapping. The default is `68`.
	pub line_width: u32,
	/// Convert smart quotes, em dashes etc with ASCII equivalents. The default
	/// is `false`.
	pub ascii_symbols: bool,
	/// Remove all comments from the document. The default is
	/// `false`.
	pub strip_comments: bool,
	/// Join multiple class assignments into one.
	/// The default is `false`.
	pub join_classes: bool,
	/// Join multiple style attributes into one. The default is `true`.
	pub join_styles: bool,
	/// Add a newline after a hard line break (`<br>`). The default is `false`.
	pub br_newline: bool,
	/// Merge adjacent `<div>` elements into one. The default is `false`.
	pub merge_divs: bool,
	/// Merge adjacent `<span>` elements into one. The default is `false`.
	pub merge_spans: bool,
}

impl Default for FormatOptions {
	fn default() -> Self {
		Self {
			line_width: 68,
			line_ending: LineEnding::Lf,
			indent: Indent::default(),
			ascii_symbols: false,
			strip_comments: false,
			join_classes: false,
			join_styles: true,
			br_newline: false,
			merge_divs: false,
			merge_spans: false,
		}
	}
}

/// Indentation options for formatting.
#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Indent {
	/// - If using spaces: Number of spaces to use for each level of
	///   indentation.
	/// - If using tabs: How aggressively indentation should be applied (it's is
	///   weird - Tidy docs aren't clear on what it does; a good default is 4 to 8).
	///
	/// Setting this to 0 will turn off indentation.
	/// The default is `4`.
	pub size: u16,
	/// Use hard tabs (`\t`) for indentation. The default is `true`.
	pub tabs: bool,
	/// Add a newline and indent before each attribute. The default is `false`.
	pub attributes: bool,
	/// Indent contents of `<![CDATA[...]]` sections. The default is `false`.
	pub cdata: bool,
}

impl Default for Indent {
	fn default() -> Self {
		Self {
			size: 4,
			tabs: true,
			attributes: false,
			cdata: false,
		}
	}
}

/// The line ending to use while formatting.
#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LineEnding {
	/// Line feed (`\n`).
	Lf,
	/// Carriage return (`\r`) - you probably don't want to use this.
	Cr,
	/// Carriage return + line feed (`\r\n`).
	CrLf,
}

pub(crate) fn reset_opts(doc: TidyDoc, xml: bool) {
	use TidyOptionId::*;
	const OPTS: &[(TidyOptionId, bool)] = &[
		(ShowFilename, false),
		(ShowInfo, false),
		(ShowMetaChange, false),
		(CoerceEndTags, false),
		(DropEmptyParas, true),
		(LowerLiterals, true),
		(Mark, false),
		(MetaCharset, false),
		(Quiet, true),
	];

	unsafe {
		let res = tidyOptResetAllToDefault(doc);
		debug_assert_eq!(res, 1, "tidyOptResetAllToDefault returned {res}");
	}

	set_bool(doc, XmlTags, xml);
	set_bool(doc, XmlOut, xml);
	set_bool(doc, XmlDecl, xml);
	set_int(doc, BodyOnly, TidyTriState::AutoState as u32);

	for &(id, yes) in OPTS {
		set_bool(doc, id, yes);
	}

	let res = unsafe { tidySetCharEncoding(doc, b"utf8\0".as_ptr() as *const i8) };
	debug_assert_eq!(res, 0, "tidySetCharEncoding returned {res}");
}

#[inline]
pub(crate) fn set_int(doc: TidyDoc, id: TidyOptionId, val: u32) {
	let res = unsafe { tidyOptSetInt(doc, id, val as _) };
	debug_assert_eq!(res, 1, "tidyOptSetInt({id:?}, {val}) returned {res}");
}

#[inline]
pub(crate) fn set_bool(doc: TidyDoc, id: TidyOptionId, yes: bool) {
	let res = unsafe { tidyOptSetBool(doc, id, yes as _) };
	debug_assert_eq!(res, 1, "tidyOptSetBool({id:?}, {yes}) returned {res}");
}

impl FormatOptions {
	pub(crate) fn apply(&self, doc: TidyDoc) {
		use TidyOptionId::*;
		let ending = match self.line_ending {
			LineEnding::Lf => TidyLineEnding::LF as u32,
			LineEnding::CrLf => TidyLineEnding::CRLF as u32,
			LineEnding::Cr => TidyLineEnding::CR as u32,
		};
		set! {
			doc, set_int,
		Newline = ending,
		WrapLen = self.line_width,
		IndentSpaces = self.indent.size as u32,
		MergeDivs = self.merge_divs as _,
		MergeSpans = self.merge_spans as _,
		IndentContent = (self.indent.size > 0) as _,
		}

		set! {
				doc, set_bool,
			IndentAttributes = self.indent.attributes,
			IndentCdata = self.indent.cdata,
			PPrintTabs = self.indent.tabs,
				// Other
		JoinStyles = self.join_styles,
			HideComments = self.strip_comments,
			JoinClasses = self.join_classes,
			MakeBare = self.ascii_symbols,
			BreakBeforeBR = self.br_newline,
		}
	}
}
