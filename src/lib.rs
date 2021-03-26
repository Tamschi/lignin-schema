#![forbid(unsafe_code)]
#![no_std]
#![doc(html_root_url = "https://docs.rs/lignin-schema/0.0.4")]
#![warn(clippy::pedantic)]
#![allow(non_camel_case_types)]

#[cfg(doctest)]
pub mod readme {
	doc_comment::doctest!("../README.md");
}

mod private {
	use crate::{NoContent, SomeContent};

	pub trait Sealed {}
	impl Sealed for NoContent {}
	impl Sealed for SomeContent {}
}
use private::Sealed;

pub trait MaybeContent: Sealed {}
pub struct NoContent;
pub struct SomeContent;
impl MaybeContent for NoContent {}
impl MaybeContent for SomeContent {}

macro_rules! element_attribute_trait {
	($name:ident) => {
		pub trait $name: Sealed {}
		impl<T> $name for T where T: GlobalAttribute {}
	};
}

macro_rules! elements {
	($(
		$(-$(-$deprecated:tt)?)?
		$name:ident
	),*$(,)?) => {$(
		$(
			#[deprecated = "To quote MDN: Warning: \"These are old HTML elements which are deprecated and should not be used. You should never use them in new projects, and should replace them in old projects as soon as you can. They are listed here for informational purposes only.\""]
			$(compile_error!($deprecated))?
		)?
		#[inline(always)]
		#[must_use]
		pub fn $name(_has_content: &dyn MaybeContent, _attributes: &[&dyn $name]) -> &'static str {
			heck_but_macros::stringify_SHOUTY_SNEK_CASE!($name)
		}

		element_attribute_trait!($name);
	)*};
}

macro_rules! void_elements {
	($(
		$(-$(-$deprecated:tt)?)?
		$name:ident
	),*$(,)?) => {$(
		$(
			#[deprecated = "To quote MDN: Warning: \"These are old HTML elements which are deprecated and should not be used. You should never use them in new projects, and should replace them in old projects as soon as you can. They are listed here for informational purposes only.\""]
			$(compile_error!($deprecated))?
		)?
		#[inline(always)]
		#[must_use]
		pub const fn $name(_: &NoContent) -> &'static str {
			heck_but_macros::stringify_SHOUTY_SNEK_CASE!($name)
		}

		element_attribute_trait!($name);
	)*};
}

macro_rules! attributes {
	{$namespace:ident=>
		$(
			$(-$(-$deprecated:tt)?)?
			$name:ident on
			$([$(
				$(-$(-$deprecated_impl:tt)?)?
				$element:ident
			),*$(,)?])?
			$(all $($global_marker:ident)?)?
		),*$(,)?
	} => {$(
		$(
			#[deprecated = "TODO"]
			$(compile_error!($deprecated))?
		)?
		pub struct $name;
		#[allow(deprecated)]
		impl $name {
			#[inline(always)]
			#[must_use]
			pub const fn attribute_name() -> &'static str {
				heck_but_macros::stringify_kebab_case!($name)
			}
		}
		#[allow(deprecated)]
		impl Sealed for $name {}
		$($(
			$(
				#[allow(useless_deprecated)] //TODO: Where else to put this?
				#[deprecated = "TODO"]
				$(compile_error!($deprecated_impl))?
			)?
			#[allow(deprecated)]
			impl crate::$namespace::$element for $name {}
		)*)?
		$(
			impl crate::$namespace::GlobalAttribute for $name {}
			$(compile_error!($global_marker))?
		)?
	)*};
}

pub mod html {
	use crate::{MaybeContent, NoContent, Sealed};

	pub trait GlobalAttribute: Sealed {}

	//SEE: https://developer.mozilla.org/en-US/docs/Web/HTML/Element
	// Main root
	elements!(html);
	// Document metadata
	elements!(head, style, title);
	void_elements!(base, link, meta);
	// Sectioning root
	elements!(body);
	// Content sectioning
	elements!(
		address, article, aside, footer, header, h1, h2, h3, h4, h5, h6, hgroup, main, nav, section
	);
	// Text content
	elements!(blockquote, dd, div, dl, dt, figcaption, figure, hr, li, /*main,*/ ol, p, pre, ul);
	// Inline text semantics
	elements!(
		a, abbr, b, bdi, bdo, cite, code, data, dfn, em, i, kbd, mark, q, rb, rp, rt, rtc, ruby, s,
		samp, small, span, strong, sub, sup, time, u, var
	);
	void_elements!(br, wbr);
	// Image and multimedia
	elements!(audio, map, video);
	void_elements!(area, img, track);
	// Embedded content
	elements!(iframe, object, picture);
	void_elements!(embed, param, source);
	// Scripting
	elements!(canvas, noscript, script);
	// Demarcating edits
	elements!(del, ins);
	// Table content
	elements!(caption, colgroup, table, tbody, td, tfoot, th, thead, tr);
	void_elements!(col);
	// Forms
	elements!(
		button, datalist, fieldset, form, label, legend, meter, optgroup, option, progress, select,
		textarea
	);
	void_elements!(input);
	// Interactive elements
	elements!(details, dialog, menu, summary);
	// Web components
	elements!(slot, template);

	// Deprecated
	elements!(
		-acronym, -applet, -big, -blink, -center, -content, -dir, -element, -font, -frameset,
		-listing, -marquee, -multicol, -nobr, -noembed, -noframes, -plaintext, -shadow, -strike,
		-tt, -xmp,
	);
	void_elements!(
		-basefont, -bgsound, -command, -frame,
		-image, // The spec doesn't actually say whether this allows content.
		-isindex, -keygen, -menuitem, -nextid, -spacer
	);

	pub mod attributes {
		use super::Sealed;

		attributes! {html=>
			accept on [input, -form],
			//accept_charset on [form],
			accesskey on all,
			action on [form],
			align on [applet, caption, col, colgroup, hr, iframe, img, table, tbody, td, tfoot, th, thead, tr],
			allow on [iframe],
			alt on [applet, area, img, input],
			//r#async on [script],
			autocapitalize on all,
			autocomplete on [form, input, select, textarea],
			autofocus on [button, input, keygen, select, textarea],
			autoplay on [audio, video],
			-background on [body, table, td, th],
			-bgcolor on [body, col, colgroup, marquee, table, tbody, tfoot, td, th, tr],
			-border on [img, object, table],
			buffered on [audio, video],
			capture on [input],
			challenge on [keygen],
			charset on [meta, script],
			checked on [command, input],
			cite on [blockquote, del, ins, q],
			class on all,
			code on [applet],
			codebase on [applet],
			-color on [basefont, font, hr],
			cols on [textarea],
		}
	}
}
