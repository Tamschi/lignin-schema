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

pub trait Global: Sealed {}
impl<T> html::Global for T where T: Global {}
impl<T> svg::Global for T where T: Global {}

pub trait MaybeContent: Sealed {}
pub struct NoContent;
pub struct SomeContent;
impl MaybeContent for NoContent {}
impl MaybeContent for SomeContent {}

macro_rules! element_attribute_trait {
	(
		$(-$(-$deprecated:tt)?)?
		$name:ident
	) => {
		$(
			#[deprecated = "To quote MDN: Warning: \"These are old HTML elements which are deprecated and should not be used. You should never use them in new projects, and should replace them in old projects as soon as you can. They are listed here for informational purposes only.\""]
			$(compile_error!($deprecated))?
		)?
		pub trait $name: Sealed {}
		#[allow(deprecated)]
		impl<T> $name for T where T: Global {}
	};
}

macro_rules! elements {
	($(
		$(-$(-$deprecated:tt)?)?
		$name:ident
	),*$(,)?) => {$(
		#[allow(deprecated)]
		$(
			#[deprecated = "To quote MDN: Warning: \"These are old HTML elements which are deprecated and should not be used. You should never use them in new projects, and should replace them in old projects as soon as you can. They are listed here for informational purposes only.\""]
			$(compile_error!($deprecated))?
		)?
		#[inline(always)]
		#[must_use]
		pub fn $name(_has_content: &dyn MaybeContent, _attributes: &[&dyn $name]) -> &'static str {
			heck_but_macros::stringify_SHOUTY_SNEK_CASE!($name)
		}

		element_attribute_trait!(
			$(
				-
				$(compile_error!($deprecated))?
			)?
			$name
		);
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
		pub fn $name(_: &NoContent, _attributes: &[&dyn $name]) -> &'static str {
			heck_but_macros::stringify_SHOUTY_SNEK_CASE!($name)
		}

		element_attribute_trait!($name);
	)*};
}

macro_rules! attributes {
	{$namespace:ident=>
		$(
			$(-$(-$deprecated:tt)?)?
			$(*$(*$experimental:tt)?)?
			$(!$(!$obsolete:tt)?)?
			$name:ident on
			$([$(
				$(-$(-$deprecated_impl:tt)?)?
				$element:ident
			),*$(,)?])?
			$(all $($global_marker:ident)?)?
		),*$(,)?
	} => {$(
		$(
			#[deprecated = "This deprecated API should no longer be used, but will probably still work."]
			/// `deprecated`
			$(compile_error!($deprecated))?
		)?
		$(
			#[deprecated = "This is an experimental API that should not be used in production code."]
			/// `experimental`
			$(compile_error!($experimental))?
		)?
		$(
			#[deprecated = "This is an obsolete API and is no longer guaranteed to work."]
			/// `obsolete`
			$(compile_error!($obsolete))?
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
			impl crate::$namespace::elements::$element for $name {}
		)*)?
		$(
			#[allow(deprecated)]
			impl crate::$namespace::Global for $name {}
			$(compile_error!($global_marker))?
		)?
	)*};
}

pub mod html {
	use crate::Sealed;

	pub trait Global: Sealed {}

	/// See <https://developer.mozilla.org/en-US/docs/Web/HTML/Element>.
	pub mod elements {
		use super::Global;
		use crate::{MaybeContent, NoContent, Sealed};

		// Main root
		elements!(html);
		// Document metadata
		elements!(head, style, title);
		void_elements!(base, link, meta);
		// Sectioning root
		elements!(body);
		// Content sectioning
		elements!(
			address, article, aside, footer, header, h1, h2, h3, h4, h5, h6, hgroup, main, nav,
			section
		);
		// Text content
		elements!(
			blockquote, dd, div, dl, dt, figcaption, figure, hr, li, /*main,*/ ol, p, pre, ul
		);
		// Inline text semantics
		elements!(
			a, abbr, b, bdi, bdo, cite, code, data, dfn, em, i, kbd, mark, q, rb, rp, rt, rtc,
			ruby, s, samp, small, span, strong, sub, sup, time, u, var
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
			button, datalist, fieldset, form, label, legend, meter, optgroup, option, output,
			progress, select, textarea
		);
		void_elements!(input);
		// Interactive elements
		elements!(details, dialog, menu, summary);
		// Web components
		elements!(slot, template);

		// Deprecated
		elements!(
			-acronym, -applet, -big, -blink, -center, -content, -dir, -element, -font, -frameset,
			-listing, -marquee, -multicol, -nobr, -noembed, -noframes, -plaintext, -shadow,
			-strike, -tt, -xmp,
		);
		void_elements!(
			-basefont, -bgsound, -command, -frame,
			-image, // The spec doesn't actually say whether this allows content.
			-isindex, -keygen, -menuitem, -nextid, -spacer
		);
	}

	/// See <https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes>.
	pub mod attributes {
		use super::Sealed;

		pub use crate::aria_attributes::*;

		attributes! {html=>
			accept on [input, -form],
			// accept_charset on [form],
			accesskey on all,
			action on [form],
			align on [applet, caption, col, colgroup, hr, iframe, img, table, tbody, td, tfoot, th, thead, tr],
			allow on [iframe],
			alt on [applet, area, img, input],
			// r#async on [script],
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
			colspan on [td, th],
			content on [meta],
			contenteditable on all,
			contextmenu on all,
			controls on [audio, video],
			coords on [area],
			crossorigin on [audio, img, link, script, video],
			*csp on [iframe],
			data on [object],
			//data-* on all,
			datetime on [del, ins, time],
			decoding on [img],
			default on [track],
			defer on [script],
			dir on all,
			dirname on [input, textarea],
			disabled on [button, command, fieldset, input, keygen, optgroup, option, select, textarea],
			download on [a, area],
			draggable on all,
			enctype on [form],
			*enterkeyhint on all, // <textarea>, contenteditable
			// r#for on [label, output],
			form on [button, fieldset, input, keygen, label, meter, object, output, progress, select, textarea],
			formaction on [input, button],
			formenctype on [button, input],
			formmethod on [button, input],
			formnovalidate on [button, input],
			formtarget on [button, input],
			headers on [td, th],
			height on [canvas, embed, iframe, img, input, object, video], // and deprecated `on all`, but this can't be expressed without min_specialization.
			hidden on all,
			high on [meter],
			href on [a, area, base, link],
			hreflang on [a, area, link],
			// http_equiv on [meta],
			icon on [command],
			id on all,
			*importance on [iframe, img, link, script],
			integrity on [link, script],
			-intrinsicsize on [img],
			inputmode on all, // <textarea>, contenteditable
			ismap on [img],
			itemprop on all,
			keytype on [keygen],
			kind on [track],
			label on [optgroup, option, track],
			lang on all,
			-language on [script],
			*loading on [img, iframe],
			list on [input],
			// r#loop on [audio, bgsound, marquee, video],
			low on [meter],
			!manifest on [html],
			max on [input, meter, progress],
			maxlength on [input, textarea],
			minlength on [input, textarea],
			media on [a, area, link, source, style],
			method on [form],
			min on [input, meter],
			multiple on [input, select],
			muted on [audio, video],
			name on [button, form, fieldset, iframe, input, keygen, object, output, select, textarea, map, meta, param],
			novalidate on [form],
			open on [details],
			optimum on [meter],
			pattern on [input],
			ping on [a, area],
			placeholder on [input, textarea],
			poster on [video],
			preload on [audio, video],
			radiogroup on [command],
			readonly on [input, textarea],
			referrerpolicy on [a, area, iframe, img, link, script],
			rel on [a, area, link],
			required on [input, select, textarea],
			reversed on [ol],
			rows on [textarea],
			rowspan on [td, th],
			sandbox on [iframe],
			scope on [th],
			!scoped on [style],
			selected on [option],
			shape on [a, area],
			size on [input, select],
			sizes on [link, img, source],
			slot on all,
			span on [col, colgroup],
			spellcheck on all,
			src on [audio, embed, iframe, img, input, script, source, track, video],
			srcdoc on [iframe],
			srclang on [track],
			srcset on [img, source],
			start on [ol],
			step on [input],
			style on all,
			-summary on [table],
			tabindex on all,
			target on [a, area, base, form],
			title on all,
			translate on all,
			// r#type on [button, input, command, embed, object, script, source, style, menu],
			usemap on [img, input, object],
			value on [button, data, input, li, meter, option, progress, param],
			width on [canvas, embed, iframe, img, input, object, video], // and deprecated `on all`, but this can't be expressed without min_specialization.
			wrap on [textarea],
		}
	}
}

// Replace element macros due to different name casing.
macro_rules! elements {
	($(
		$(-$(-$deprecated:tt)?)?
		$name:ident
	),*$(,)?) => {$(
		#[allow(deprecated)]
		#[allow(non_snake_case)]
		$(
			#[deprecated = "To quote MDN: Warning: \"These are old SVG elements which are deprecated and should not be used. You should never use them in new projects, and should replace them in old projects as soon as you can. They are listed here for informational purposes only.\""]
			$(compile_error!($deprecated))?
		)?
		#[inline(always)]
		#[must_use]
		pub fn $name(_has_content: &dyn MaybeContent, _attributes: &[&dyn $name]) -> &'static str {
			stringify!($name)
		}

		element_attribute_trait!(
			$(
				-
				$(compile_error!($deprecated))?
			)?
			$name
		);
	)*};
}
macro_rules! void_elements {
	($(
		$(-$(-$deprecated:tt)?)?
		$name:ident
	),*$(,)?) => {$(
		#[allow(non_snake_case)]
		$(
			#[deprecated = "To quote MDN: Warning: \"These are old SVG elements which are deprecated and should not be used. You should never use them in new projects, and should replace them in old projects as soon as you can. They are listed here for informational purposes only.\""]
			$(compile_error!($deprecated))?
		)?
		#[inline(always)]
		#[must_use]
		pub fn $name(_: &NoContent, _attributes: &[&dyn $name]) -> &'static str {
			stringify!($name)
		}

		element_attribute_trait!($name);
	)*};
}

pub mod svg {
	use crate::Sealed;

	pub trait Global: Sealed {}

	/// See <https://developer.mozilla.org/en-US/docs/Web/SVG/Element>.
	pub mod elements {
		use super::Global;
		use crate::{MaybeContent, NoContent, Sealed};

		// Animation elements
		elements!(
			-animateColor,
			animateMotion,
			animateTransform,
			discard,
			mpath,
			set
		);
		void_elements!(animate,); //?

		// Basic shapes
		elements!(circle, ellipse, line, polygon, polyline, rect);

		// Container elements
		//BUG: Fix casing so that it's `"missing-glyph"`!
		elements!(
			a,
			defs,
			g,
			marker,
			mask,
			missing_glyph,
			pattern,
			svg,
			switch,
			symbol,
			unknown,
		);

		// Descriptive elements
		elements!(desc, metadata, title);

		// Filter primitive elements
		elements!(
			feBlend,
			feColorMatrix,
			feComponentTransfer,
			feComposite,
			feConvolveMatrix,
			feDiffuseLighting,
			feDisplacementMap,
			feDropShadow,
			feFlood,
			feFuncA,
			feFuncB,
			feFuncG,
			feFuncR,
			feGaussianBlur,
			feImage,
			feMerge,
			feMergeNode,
			feMorphology,
			feOffset,
			feSpecularLighting,
			feTile,
			feTurbulence,
		);

		// Font elements
		//BUG: Fix casing!
		elements!(-font, -font_face, -font_face_src, -font_face_uri);
		void_elements!(-font_face_format, -font_face_name, -hkern, -vkern);

		// Gradient elements
		elements!(linearGradient, meshgradient, radialGradient, stop); //TODO: Check casing on `meshgradient`.
	}

	/// See <https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes>.
	pub mod attributes {
		use super::Sealed;

		pub use crate::aria_attributes::*;

		attributes! {svg=>
		}
	}
}

pub trait AriaAttribute: Sealed {}
impl<T> Global for T where T: AriaAttribute {}

/// See <https://www.w3.org/TR/wai-aria-1.1/#state_prop_def>.
pub mod aria_attributes {
	use crate::{AriaAttribute, Sealed};

	macro_rules! aria_attributes {
		{$(
			$(-$(-$deprecated:tt)?)?
			$(*$(*$experimental:tt)?)?
			$(!$(!$obsolete:tt)?)?
			$name:ident
		),*$(,)?} => {$(
			$(
				#[deprecated = "This deprecated API should no longer be used, but will probably still work."]
				/// `deprecated`
				$(compile_error!($deprecated))?
			)?
			$(
				#[deprecated = "This is an experimental API that should not be used in production code."]
				/// `experimental`
				$(compile_error!($experimental))?
			)?
			$(
				#[deprecated = "This is an obsolete API and is no longer guaranteed to work."]
				/// `obsolete`
				$(compile_error!($obsolete))?
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
			#[allow(deprecated)]
			impl AriaAttribute for $name {}
		)*};
	}

	aria_attributes!(
		role,
		// aria_activedescendant,
		// aria_atomic,
		// aria_autocomplete,
		// aria_busy,
		// aria_checked,
		// aria_colcount,
		// aria_colindex,
		// aria_colspan,
		// aria_controls,
		// aria_current,
		// aria_describedby,
		// aria_details,
		// aria_disabled,
		// aria_dropeffect,
		// aria_errormessage,
		// aria_flowto,
		// aria_grabbed,
		// aria_haspopup,
		// aria_hidden,
		// aria_invalid,
		// aria_keyshortcuts,
		// aria_label,
		// aria_labelledby,
		// aria_level,
		// aria_live,
		// aria_modal,
		// aria_multiline,
		// aria_multiselectable,
		// aria_orientation,
		// aria_owns,
		// aria_placeholder,
		// aria_posinset,
		// aria_pressed,
		// aria_readonly,
		// aria_relevant,
		// aria_required,
		// aria_roledescription,
		// aria_rowcount,
		// aria_rowindex,
		// aria_rowspan,
		// aria_selected,
		// aria_setsize,
		// aria_sort,
		// aria_valuemax,
		// aria_valuemin,
		// aria_valuenow,
		// aria_valuetext,
	);
}

pub trait YesNo: Sealed {
	const IS_YES: bool;
}
pub struct Yes;
impl Sealed for Yes {}
impl YesNo for Yes {
	const IS_YES: bool = true;
}
pub struct No;
impl Sealed for No {}
impl YesNo for No {
	const IS_YES: bool = false;
}

pub trait Event: Sealed {
	const NAME: &'static str;
	type Bubbles: YesNo;
	type Cancelable: YesNo;
}

/// See <https://developer.mozilla.org/en-US/docs/Web/Events#event_listing>.
///
/// This module only covers associations on [***Element***](https://developer.mozilla.org/en-US/docs/Web/API/Element)
/// and some derived types.
///
/// # Legend
///
/// ## ↕️ - Bubbles
///
/// ## ⏹️ - Cancelable
///
/// ## 🌐 - Global(-ish)
///
/// The event is available on any [***Element***](https://developer.mozilla.org/en-US/docs/Web/API/Element).
///
/// Implied by [↕️ - Bubbles](#---bubbles).
pub mod events {
	use super::{Event, Sealed, Yes, No, Global};

	macro_rules! events {
		($(
			$(- $(!$deprecated:tt)?)?
			$name:ident
			$(bubbles $(!$bubbles:tt)?)?
			$(on
				$([$($namespace:ident::$element:ident),*$(,)?])?
				$(all $(!$all:tt)?)?
			)?
			$(cancelable $(!$cancelable:tt)?)?
			$(non-standard $(!$non_standard:tt)?)?
	
		),*$(,)?) => {$(
			
			$(
				#[deprecated = "This feature is no longer recommended."]
				/// `deprecated`
				$(compile_error!($deprecated))?
			)?
			$(
				#[deprecated = "This feature is non-standard and is not on a standards track."]
				/// `non-standard`
				$(compile_error!($non_standard))?
			)?
			$(
				/// ↕️
				$(compile_error!($bubbles))?
			)?
			$(
				/// ⏹️
				$(compile_error!($cancelable))?
			)?
			$($(
				/// 🌐
				$(compile_error!($all))?
			)?)?
			#[allow(clippy::upper_case_acronyms)]
			pub struct $name;
			#[allow(deprecated)]
			impl Sealed for $name {}
			#[allow(deprecated)]
			impl Event for $name {
				const NAME: &'static str = stringify!($name);
				$(
					type Bubbles = Yes;
					$(compile_error!($bubbles))?
					#[cfg(FALSE)]
				)?
				type Bubbles = No;
	
				$(
					type Cancelable = Yes;
					$(compile_error!($cancelable))?
					#[cfg(FALSE)]
				)?
				type Cancelable = No;
			}
			$(
				#[allow(deprecated)]
				impl Global for $name {}
				$(compile_error!($bubbles))?
			)?
			$(
				$($(
					impl crate::$namespace::elements::$element for $name {}
				)*)?
				$(
					impl Global for $name {}
					$(compile_error!($all))?
				)?
			)?
		)*};
	}

	events!(
		// Element
		afterscriptexecute bubbles non-standard,
		auxclick bubbles cancelable,
		beforescriptexecute bubbles cancelable non-standard,
		blur on all,
		click bubbles cancelable,
		compositionend bubbles cancelable,
		compositionstart bubbles cancelable,
		compositionupdate bubbles cancelable,
		contextmenu bubbles cancelable,
		copy bubbles cancelable,
		cut bubbles cancelable,
		dblclick bubbles cancelable,
		-DOMActivate bubbles cancelable,
		DOMMouseScroll bubbles cancelable non-standard,
		error on all,
		focusin bubbles,
		focusout bubbles,
		focus on all,
		fullscreenchange bubbles,
		fullscreenerror bubbles,
		gesturechange on [] non-standard,
		gestureend on [] non-standard,
		gesturestart on [] non-standard,
		keydown bubbles cancelable,
		-keypress bubbles cancelable,
		keyup bubbles cancelable,
	);
}
