#![forbid(unsafe_code)]
#![no_std]
#![doc(html_root_url = "https://docs.rs/lignin-schema/0.0.4")]
#![warn(clippy::pedantic)]

use heck_but_macros::stringify_SHOUTY_SNEK_CASE;
pub use lignin;

#[cfg(doctest)]
pub mod readme {
	doc_comment::doctest!("../README.md");
}

macro_rules! element {
	($name:ident) => {
		#[inline(always)]
		#[must_use]
		pub fn $name<'a>(
			attributes: &'a [lignin::Attribute<'a>],
			content: &'a [lignin::Node<'a>],
			event_bindings: &'a [lignin::EventBinding<'a>],
		) -> lignin::Element<'a> {
			lignin::Element {
				name: stringify_SHOUTY_SNEK_CASE!($name),
				attributes,
				content,
				event_bindings,
			}
		}
	};
}

macro_rules! void_element {
	($name:ident) => {
		#[inline(always)]
		#[must_use]
		pub fn $name<'a>(
			attributes: &'a [lignin::Attribute<'a>],
			_: &'a [lignin::Node<'a>; 0],
			event_bindings: &'a [lignin::EventBinding<'a>],
		) -> lignin::Element<'a> {
			lignin::Element {
				name: stringify_SHOUTY_SNEK_CASE!($name),
				attributes,
				content: &[],
				event_bindings,
			}
		}
	};
}

macro_rules! elements {
    ($($name:ident),+) => {
        $(element!($name);)+
    };
}

macro_rules! void_elements {
    ($($name:ident),+) => {
        $(void_element!($name);)+
    };
}

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

#[deprecated = "To quote MDN: Warning: \"These are old HTML elements which are deprecated and should not be used. You should never use them in new projects, and should replace them in old projects as soon as you can. They are listed here for informational purposes only.\""]
/// Don't actually use these. They're broken or could break at a moment's notice (or without notice, for that matter...).
pub mod deprecated {
	use super::stringify_SHOUTY_SNEK_CASE;
	elements!(
		acronym, applet, big, blink, center, content, dir, element, font, frameset, listing,
		marquee, multicol, nobr, noembed, noframes, plaintext, shadow, strike, tt, xmp
	);
	void_elements!(
		basefont, bgsound, command, frame,
		image, // The spec doesn't actually say whether this allows content.
		isindex, keygen, menuitem, nextid, spacer
	);
}
