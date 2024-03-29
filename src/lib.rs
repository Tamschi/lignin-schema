#![forbid(unsafe_code)]
#![no_std]
#![doc(html_root_url = "https://docs.rs/lignin-schema/0.0.4")]
#![warn(clippy::pedantic)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![allow(non_camel_case_types)]

#[cfg(doctest)]
pub mod readme {
	doc_comment::doctest!("../README.md");
}

mod private {
	pub trait Sealed {}
}
use private::Sealed;

pub trait Global<Aspect: ?Sized>: Sealed {}

pub trait HasContent: Sealed {
	#[inline(always)]
	fn static_validate_on(_: Self)
	where
		Self: Sized,
	{
		// Intentionally left empty.
	}
}

#[deprecated = "Don't specify aspects directly, which may be too limiting. Generic type parameter resolution should never fail as there are no overloaded implementations."]
pub mod aspects {
	enum Vacant {}
	pub struct Content(Vacant);
	pub struct Attribute(Vacant);
	pub struct Event(Vacant);
}
#[allow(deprecated)]
use aspects::Attribute;

macro_rules! element_common {
	(
		$(#[$($attribute_token:tt)*])*
		$(-$(-$deprecated:tt)?)?
		$name:ident {
			tag_name: $tag_name:expr,
		}
	) => {
		$(
			#[deprecated = "To quote MDN: Warning: \"These are old […] elements which are deprecated and should not be used. You should never use them in new projects, and should replace them in old projects as soon as you can. They are listed here for informational purposes only.\""]
			$(compile_error!($deprecated))?
		)?
		$(#[$($attribute_token)*])*
		pub struct $name;

		#[allow(deprecated)]
		impl $name {
			#[must_use]
			pub const TAG_NAME: &'static str = $tag_name;
		}

		#[allow(deprecated)]
		impl Sealed for $name {}
		#[allow(deprecated)]
		impl<Aspect: ?Sized> Global<Aspect> for $name {}
		#[allow(deprecated)]
		impl<Aspect: ?Sized> crate::Global<Aspect> for $name {}
	};
}

macro_rules! elements {
	{$(
		$(#[$($attribute_token:tt)*])*
		$(-$(@$deprecated:tt)?)?
		$(/$(@$empty:tt)?)?
		$name:ident
	),*$(,)?} => {$(
		element_common! {
			$(
				$(@$empty)?
				/// [***Empty.***](https://developer.mozilla.org/en-US/docs/Glossary/empty_element)
			)?
			$(#[$($attribute_token)*])*
			$(-$(@$deprecated)?)? $name {
				tag_name: heck_but_macros::stringify_SHOUTY_SNEK_CASE!($name),
			}
		}

		$(
			$(@$empty)?
			#[cfg(FALSE)]
		)?
		#[allow(deprecated)]
		impl HasContent for $name {}
	)*};
}

macro_rules! attributes {
	{$namespace:ident=>
		$(
			$(#[$($attribute_token:tt)*])*
			$(-$(@$deprecated:tt)?)?
			$(*$(@$experimental:tt)?)?
			$(!$(@$obsolete:tt)?)?
			$(%$(@$non_standard:tt)?)?
			$name:ident on
			$([$(
				$(#[$($impl_attribute_token:tt)*])*
				$(-$(@$deprecated_impl:tt)?)?
				$(%$(@$non_standard_impl:tt)?)?
				$element:ident
			),*$(,)?])?
			$(all $($global_marker:ident)?)?
		),*$(,)?
	} => {$(
		$(
			#[deprecated = "deprecated - probably still supported, but discouraged (usually in favor of a better alternative)."]
			/// `deprecated`
			$(@$deprecated)?
		)?
		$(
			#[deprecated = "experimental - not for production code and likely not well supported yet."]
			/// `experimental`
			$(@$experimental)?
		)?
		$(
			#[deprecated = "obsolete - most likely removed from most browsers that used to support it."]
			/// `obsolete`
			$(@$obsolete)?
		)?
		$(
			#[deprecated = "non-standard - may behave differently between browsers or not work at all."]
			/// `non-standard`
			$(@$non_standard)?
		)?
		#[allow(deprecated)]
		$(#[$($attribute_token)*])*
		pub trait $name<Aspect: ?Sized = Attribute>: Sealed {
			#[inline(always)]
			fn static_validate_on(_: Self) where Self: Sized {
				// Intentionally left blank.
			}
		}
		#[allow(deprecated)]
		impl dyn $name<Attribute> {
			#[must_use]
			pub const NAME: &'static str = heck_but_macros::stringify_kebab_case!($name);
		}
		$($(
			$(
				/// `deprecated`
				//TODO: Is there a way to deprecate specific trait implementations?
				$(@$deprecated_impl)?
			)?
			$(
				/// `non-standard`
				//TODO: Is there a way to deprecate specific trait implementations?
				$(@$non_standard_impl)?
			)?
			#[allow(deprecated)]
			$(#[$($impl_attribute_token)*])*
			impl $name<Attribute> for crate::$namespace::elements::$element {
				#[inline(always)]
				fn static_validate_on(_: Self) {
					// Intentionally left blank.
				}
			}
		)*)?
		$(
			#[allow(deprecated)]
			impl<T: crate::$namespace::Global<Attribute>> $name<Attribute> for T {}
			$(compile_error!($global_marker))?
		)?
	)*};
}

pub mod html {
	use crate::Sealed;

	pub trait Global<Aspect: ?Sized>: Sealed {}

	/// See <https://developer.mozilla.org/en-US/docs/Web/HTML/Element>.
	pub mod elements {
		use super::Global;
		#[allow(deprecated)]
		use crate::{HasContent, Sealed};

		// When you edit an element, also move it to its alphabetically-ordered position.
		// Use a separate commit if it already had documentation or if you change its modifiers!
		elements!(

			/// Document-unique content and sectioning root.
			///
			/// See <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/body>.
			///
			/// # Accessibility
			///
			/// The default [`role`](`super::attributes::role`) of this element is [***document***](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles/Document_Role).
			///
			/// No alternative roles are permitted.
			///
			/// # Constraints
			///
			/// [`<body>`](`body`) may occur only as second child [***element***](https://developer.mozilla.org/en-US/docs/Glossary/Element)
			/// of an [`<html>`](`html`) element, after [`<head>`](`head`).
			///
			/// # Scripting
			///
			/// The [`<body>`](`body`) element is available through the [***Document.body***](https://developer.mozilla.org/en-US/docs/Web/API/Document/body)
			/// property.
			///
			/// # Styling
			///
			/// ## Overscroll
			///
			/// In browsers that support [overscroll](https://developer.mozilla.org/en-US/docs/Web/CSS/overscroll-behavior),
			/// the main scroll container is usually [`<html>`](`html`).
			///
			/// By making sure the [`<body>`](`body`) is large enough to cover the viewport, you can style the overscroll area independently without extra elements:
			///
			/// ```css
			/// html {
			///     background: …;
			/// }
			///
			/// body {
			///     min-height: 100vh;
			///     background: …;
			/// }
			/// ```
			///
			/// ## The Notch / Non-Rectangular Displays
			///
			/// > The styling situation in this regard is a bit too messy right now for me to give a recommendation,
			/// > but such devices should still be taken into account when possible.
			/// >
			/// > (If you have experience in this regard, [please contribute](https://github.com/Tamschi/lignin-schema).)
			///
			/// # Optional Tags
			///
			/// The [`<body>`](`body`) element can be implied in serialized HTML as follows:
			///
			/// ## Start Tag
			///
			/// The start tag can be omitted iff no attributes are present and iff the first child node is **not** one of the following:
			///
			/// * a comment
			/// * a [`<script>`](`script`) or [`<style>`](`style`) element,
			/// * a text node starting with a space character.
			///
			/// ## End Tag
			///
			/// Can be omitted iff **both** of the following are true:
			///
			/// * The [`<body>`](`body`) has content or a start tag, and
			/// * it is not immediately followed by a comment.
			body,

			/// Document-unique metadata header.
			///
			/// See <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/head>.
			///
			/// This element is intended only for machine-readable metadata.
			/// For human-visible metadata, use [`<header>`](`header`) instead.
			///
			/// In HTML5-compliant browsers, [`<head>`](`head`) is created automatically in the DOM even if completely absent from markup. **Very** old browsers may behave differently here.
			///
			/// # Accessibility
			///
			/// This element has neither a default [`role`](`super::attributes::role`) nor permitted alternatives.
			///
			/// # Constraints
			///
			/// [`<head>`](`head`) may occur only as first child [***element***](https://developer.mozilla.org/en-US/docs/Glossary/Element)
			/// of an [`<html>`](`html`) element.
			///
			/// # Optional Tags
			///
			/// The [`<head>`](`head`) element can be implied in serialized HTML as follows:
			///
			/// ## Start Tag
			///
			/// The start tag can be omitted iff this element's first child node is an [***element***](https://developer.mozilla.org/en-US/docs/Glossary/Element).
			///
			/// > Note that "child nodes" can include text containing only whitespace!
			/// > Omitting `<head>` from serialized HTML will change the resulting DOM structure unless the next element follows **immediately**.
			///
			/// ## End Tag
			///
			/// Can be omitted iff this element's next sibling is neither a comment nor a text node starting with a space character.
			head,

			/// HTML document root.
			///
			/// See <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/html>.
			///
			/// # Accessibility
			///
			/// This element has neither a default [`role`](`super::attributes::role`) nor permitted alternatives.
			///
			/// While optional, you should specify the [`lang`](`super::attributes::lang`) attribute on this element according to the majority of content on the page.
			/// Prefer this over setting it on [`<body>`](`body`), as this way also the metadata in [`<head>`](`head`), like for example [`<title>`](`title`), is affected accordingly.
			///
			/// > The [`lang`](`super::attributes::lang`) attribute can be set on **any** HTML element, so don't be afraid to be more specific for parts of the page if the content is mixed-language.
			///
			/// # Constraints
			///
			/// [`<html>`](`html`) must occur exactly once as single root element of an HTML document.
			///
			/// > However, sibling comments and *maybe* whitespace text nodes appear to be possible.
			///
			/// # Optional Tags
			///
			/// The [`<html>`](`html`) element can be implied in serialized HTML as follows:
			///
			/// ## Start Tag
			///
			/// The start tag can be omitted iff this element's first child node is **not** a comment.
			///
			/// ## End Tag
			///
			/// Can be omitted iff this element's next sibling is **not** a comment.
			html,

			style, title, /base, /link, /meta, address, article, aside, footer,
			header, h1, h2, h3, h4, h5, h6, hgroup, main, nav, section,
			blockquote, dd,

			/// A generic layout container without semantic meaning, by default rendered with [***display: block***](https://developer.mozilla.org/en-US/docs/Web/CSS/display#outside).
			///
			/// See <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/div>.
			///
			/// # Accessibility
			///
			/// A [`<div>`](`div`) itself (but not necessarily its content) is by default completely ignored when building the accessibility tree.
			///
			/// It's generally possible to adjust this behavior freely by applying [`aria_attributes`](`crate::aria_attributes`) directly to the element,
			/// but this can be complex, error prone for developers not already very familiar with the topic and confusing
			/// if the implemented behavior doesn't match what would be expected for its content and context.
			///
			/// Where available, strongly prefer semantic HTML elements with implicit [***WAI-ARIA Roles***](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles),
			/// and don't overuse landmark rules as this can make a document "noisy" and harder to skim with assistive technology.
			///
			/// # See also
			///
			/// [`<span>`](`span`), [`class<Attribute>`](`super::attributes::class`)
			div,
			dl,
			dt,
			figcaption,
			figure,

			/// Thematic break, for example as sibling between [`<p>`](`p`) elements to mark a change of scene or topic within one containing section.
			///
			/// See <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/hr>.
			///
			// cSpell:ignore orizontal
			/// [`<hr>`](`hr`) **used** to be defined as **h**orizontal **r**ule or line, but this definition has been replaced by its semantic function.
			///
			/// All element-specific attributes are deprecated or non-standard.
			///
			/// # Accessibility
			///
			/// [`<hr>`](`hr`)'s default ARIA [`role`](`super::attributes::role`) is [***separator***](https://w3c.github.io/aria/#separator).
			///
			/// Other permitted roles are [***presentation***](https://w3c.github.io/aria/#presentation)
			/// and [***none***](https://w3c.github.io/aria/#none).
			///
			/// # Styling
			///
			/// Browsers will likely still display an unspecified line by default, but CSS should be used to define the presentation from the ground up.
			///
			/// When doing so, overwrite the [***border***](https://developer.mozilla.org/en-US/docs/Web/CSS/border)
			/// CSS property, as this is normally how the default presentation is implemented,
			/// and use [***margin-block-start***](https://developer.mozilla.org/en-US/docs/Web/CSS/margin-block-start),
			/// [***margin-block-end***](https://developer.mozilla.org/en-US/docs/Web/CSS/margin-block-end),
			/// [***margin-inline-start***](https://developer.mozilla.org/en-US/docs/Web/CSS/margin-inline-start)
			/// and [***margin-inline-end***](https://developer.mozilla.org/en-US/docs/Web/CSS/margin-inline-end)
			/// to change the positioning.
			///
			/// > Unlike the more specific properties, the shorthands [***margin-block***](https://developer.mozilla.org/en-US/docs/Web/CSS/margin-block)
			/// > and [***margin-inline***](https://developer.mozilla.org/en-US/docs/Web/CSS/margin-inline)
			/// > are not supported by Edge or Safari as of 2021-04¹⁻².
			/// >
			/// > 1. <https://caniuse.com/?search=margin-block>
			/// > 2. <https://caniuse.com/?search=margin-inline>
			/hr,

			li, ol, p, pre, ul, a, abbr, b, bdi, bdo, cite, code, data, dfn, em, i, kbd, mark, q, rb, rp, rt, rtc,
			ruby, s, samp, small, span, strong, sub, sup, time, u, var,

			/// Produces a line break (carriage-return) in text.
			///
			/// See <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/br>.
			///
			/// # Accessibility
			///
			/// Do not use repetitions of this element to create spaced paragraphs.
			/// Encountering these repetitions may be confusing when using a screen-reader,
			/// which may announce each [`<br>`](`br`) separately.
			///
			/// Instead, prefer [`<p>`](`p`) elements and control [***margin***](https://developer.mozilla.org/en-US/docs/Web/CSS/margin)s
			/// via CSS.
			///
			/// # Styling
			///
			/// [`<br>`](`br`) elements generally lack dimension and visual output, which means they mostly can't be styled.
			///
			/// Instead of setting a [***margin***](https://developer.mozilla.org/en-US/docs/Web/CSS/margin)
			/// on [`<br>`](`br`), prefer changing the [***line-height***](https://developer.mozilla.org/en-US/docs/Web/CSS/line-height)
			/// of the surrounding element.
			/br,

			/wbr, audio, map, video, /area, /img, /track,
			iframe, object, picture, /embed, /param, /source,
			canvas, noscript, script,
			del, ins,
			caption, colgroup, table, tbody, td, tfoot, th, thead, tr, /col,
			button, datalist, fieldset, form, label, legend, meter, optgroup, option, output,
			progress, select, textarea, /input,
			details, dialog, menu, summary,
			slot, template,
			-acronym, -applet, -big, -blink, -center, -content, -dir, -element, -font, -frameset,
			-listing, -marquee, -multicol, -nobr, -noembed, -noframes, -plaintext, -shadow,
			-strike, -tt, -xmp,
			-/basefont, -/bgsound, -/command, -/frame,

			/// > The spec doesn't actually say whether this allows content.
			-/image,

			-/isindex, -/keygen, -/menuitem, -/nextid, -/spacer
		);
	}

	/// See <https://developer.mozilla.org/en-US/docs/Web/HTML/Attribute>.
	pub mod attributes {
		use super::Sealed;
		#[allow(deprecated)]
		use crate::aspects::Attribute;

		pub use crate::aria_attributes::*;

		attributes! {html=>
			accept on [input, -form],
			// accept_charset on [form],
			accesskey on all,
			action on [form],
			align on [
				applet, caption, col, colgroup,

				/// The alignment of the rule. Defaults to `left`.
				-hr,

				iframe, img, table, tbody, td, tfoot, th, thead, tr,
			],
			-alink on [
				/// Text color of selected hyperlinks.
				///
				/// Prefer CSS:
				///
				/// ```css
				/// *:active {
				///     color: …;
				/// }
				/// ```
				-body
			],
			allow on [iframe],
			alt on [applet, area, img, input],
			// r#async on [script],
			autocapitalize on all,
			autocomplete on [form, input, select, textarea],
			autofocus on [button, input, keygen, select, textarea],
			autoplay on [audio, video],
			-background on [
				/// URL of a background image.
				///
				/// Prefer CSS:
				///
				/// ```css
				/// body {
				///     background: url(…);
				/// }
				/// ```
				-body,

				table, td, th,
			],

			/// Use the CSS property [***background-color***](https://developer.mozilla.org/en-US/docs/Web/CSS/background-color)
			/// instead.
			///
			/// While the intended use of this attribute was with named or hexadecimal colors,
			/// [in practice various other strings are also accepted due to a lenient parsing scheme](https://stackoverflow.com/questions/8318911/why-does-html-think-chucknorris-is-a-color).
			-bgcolor on [
				/// Background color.
				///
				/// Prefer CSS:
				///
				/// ```css
				/// body {
				///     background-color: …;
				/// }
				/// ```
				-body,

				col, colgroup, marquee, table, tbody, tfoot, td, th, tr,
			],

			-border on [img, object, table],
			-bottommargin on [
				/// The bottom margin of the body.
				///
				/// Prefer CSS:
				///
				/// ```css
				/// body {
				///     margin-bottom: …;
				/// }
				/// ```
				-body,
			],
			buffered on [audio, video],
			capture on [input],
			challenge on [keygen],
			charset on [meta, script],
			checked on [command, input],
			cite on [blockquote, del, ins, q],
			class on all,
			code on [applet],
			codebase on [applet],
			-color on [
				basefont, font,

				/// The color of the rule, by name or hexadecimal value.
				%hr,
			],
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

			/// The [***URL***](https://url.spec.whatwg.org/) pointed to by a link-like element.
			///
			/// # Available on
			///
			/// [`<a>`](`super::elements::a`),
			/// [`<area>`](`super::elements::area`),
			/// [`<base>`](`super::elements::base`),
			/// [`<link>`](`super::elements::link`).
			///
			/// # Notable [`href`] patterns
			///
			/// - Fragment URLs starting with `#` to link to a section of the current page,
			/// - an absolute URL starting with `//` to remain on `http://` or `https://`,
			/// - `mailto:` followed an email address,
			/// - `tel:` followed by a phone number (primarily useful on mobile).
			///
			/// [Media fragment support appears to be limited.](https://caniuse.com/media-fragments)
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
			-leftmargin on [
				/// The left margin of the body.
				///
				/// Prefer CSS:
				///
				/// ```css
				/// body {
				///     margin-left: …;
				/// }
				/// ```
				-body,
			],
			list on [input],
			-link on [
				/// Text color of unvisited hyperlinks.
				///
				/// Prefer CSS:
				///
				/// ```css
				/// *:link {
				///     color: …;
				/// }
				/// ```
				-body
			],
			*loading on [img, iframe],
			// r#loop on [audio, bgsound, marquee, video],
			low on [meter],
			!manifest on [
				/// The URI of a resource manifest that specifies caching behavior.
				///
				/// See <https://developer.mozilla.org/en-US/docs/Web/HTML/Using_the_application_cache> (and the included removal warnings!) for more information.
				-html,
			],
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
			noshade on [
				/// Disables shading.
				-hr,
			],
			onafterprint on [
				/// `JS` Called after the user has printed the document.
				body,
			],
			onbeforeprint on [
				/// `JS` Called when the user requests printing the document.
				body,
			],
			onbeforeunload on [
				/// `JS` Called when the document is about to be unloaded.
				body,
			],
			onblur on [
				/// `JS` Called when the document loses focus.
				body,
			],
			onerror on [
				/// `JS` Called when the document fails to load properly.
				body,
			],
			onfocus on [
				/// `JS` Called when the document receives focus.
				body,
			],
			onhashchange on [
				/// `JS` Called after the fragment identifier of the document's current address changes.
				body,
			],
			*onlanguagechange on [
				/// `JS` Called after the preferred languages change.
				body,
			],
			onload on [
				/// `JS` Called when the document has finished loading.
				body,
			],
			onmessage on [
				/// `JS` Called when the document has received a message.
				body,
			],
			onoffline on [
				/// `JS` Called when network communication has failed.
				body,
			],
			ononline on [
				/// `JS` Called when network communication is available again after failure.
				body,
			],
			onpopstate on [
				/// `JS` Called when the user has navigated session history.
				body,
			],
			onredo on [
				/// `JS` Called when the user undoes an undo.
				body,
			],
			onresize on [
				/// `JS` Called after the document is resized.
				body,
			],
			onstorage on [
				//TODO: Find out what this means and elaborate.
				/// `JS` Called when the storage area has changed.
				body,
			],
			onundo on [
				/// `JS` Called after the user undoes an action via undo transaction history.
				body,
			],
			onunload on [
				/// `JS` Called when the document is being unloaded.
				body,
			],
			open on [details],
			optimum on [meter],
			pattern on [input],
			ping on [a, area],
			placeholder on [input, textarea],
			poster on [video],
			preload on [audio, video],
			-profile on [
				/// One or more space-separated metadata profile [***URI***](https://developer.mozilla.org/en-US/docs/Glossary/URI)s.
				///
				/// For more information, see <https://www.w3.org/TR/html401/struct/global.html#profiles>.
				-head,
			],
			radiogroup on [command],
			readonly on [input, textarea],
			referrerpolicy on [a, area, iframe, img, link, script],
			rel on [a, area, link],
			required on [input, select, textarea],
			reversed on [ol],
			-rightmargin on [
				/// The right margin of the body.
				///
				/// Prefer CSS:
				///
				/// ```css
				/// body {
				///     margin-right: …;
				/// }
				/// ```
				-body,
			],
			rows on [textarea],
			rowspan on [td, th],
			sandbox on [iframe],
			scope on [th],
			!scoped on [style],
			selected on [option],
			shape on [a, area],
			size on [
				input,

				/// The height of the rule in pixels.
				-hr,

				select,
			],
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
			-text on [
				/// Text color.
				///
				/// Prefer CSS:
				///
				/// ```css
				/// body {
				///     color: …;
				/// }
				/// ```
				-body,
			],
			title on all,
			-topmargin on [
				/// The top margin of the body.
				///
				/// Prefer CSS:
				///
				/// ```css
				/// body {
				///     margin-top: …;
				/// }
				/// ```
				-body,
			],
			translate on all,
			// r#type on [button, input, command, embed, object, script, source, style, menu],
			usemap on [img, input, object],
			value on [button, data, input, li, meter, option, progress, param],
			-version on [
				/// The HTML [***Document Type Definition***](https://developer.mozilla.org/en-US/docs/Glossary/Doctype) version.
				///
				/// Redundant in modern HTML documents starting with `<!DOCTYPE html>`.
				-html
			],
			-vlink on [
				/// Text color of visited hyperlinks.
				///
				/// Prefer CSS:
				///
				/// ```css
				/// *:visited {
				///     color: …;
				/// }
				/// ```
				-body
			],
			width on [
				canvas, embed, iframe, img, input,

				/// The length of the rule as pixel or percentage value.
				-hr,

				object, video,
			], // and deprecated `on all`, but this can't be expressed without min_specialization.
			wrap on [textarea],
			xmlns on [
				/// XML namespace of the document.
				///
				/// Optional in HTML (where it defaults to `http://www.w3.org/1999/xhtml`) but required in XML documents.
				html,
			],
		}
	}
}

pub mod svg {
	use crate::Sealed;

	pub trait Global<Aspect: ?Sized>: Sealed {}

	/// See <https://developer.mozilla.org/en-US/docs/Web/SVG/Element>.
	pub mod elements {
		use super::Global;
		#[allow(deprecated)]
		use crate::{HasContent, Sealed};

		// Separate element macros due to different name casing.
		macro_rules! elements {
			($(
				$(#[$($attribute_token:tt)*])*
				$(-$(-$deprecated:tt)?)?
				$name:ident
			),*$(,)?) => {$(
				element_common! {
					$(#[$($attribute_token)*])*
					$(-$($deprecated)?)? $name {
						tag_name: stringify!($name),
					}
				}

				#[allow(deprecated)]
				impl HasContent for $name {}
			)*};
		}
		macro_rules! void_elements {
			($(
				$(#[$($attribute_token:tt)*])*
				$(-$(-$deprecated:tt)?)?
				$name:ident
			),*$(,)?) => {$(
				element_common! {
					/// [***Empty.***](https://developer.mozilla.org/en-US/docs/Glossary/empty_element)
					$(#[$($attribute_token)*])*
					$(-$($deprecated)?)? $name {
						tag_name: stringify!($name),
					}
				}
			)*};
		}

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

	/// See <https://developer.mozilla.org/en-US/docs/Web/HTML/Attribute>.
	pub mod attributes {
		#[allow(unused_imports)] //TODO: Remove this once there is content here.
		use super::Sealed;

		pub use crate::aria_attributes::*;

		attributes! {svg=>
		}
	}
}

/// See <https://www.w3.org/TR/wai-aria-1.1/#state_prop_def>.
pub mod aria_attributes {
	#[allow(deprecated)]
	use crate::{Attribute, Global, Sealed};

	macro_rules! aria_attributes {
		{$(
			$(#[$($attribute_token:tt)*])*
			$(-$(-$deprecated:tt)?)?
			$(*$(*$experimental:tt)?)?
			$(!$(!$obsolete:tt)?)?
			$name:ident
		),*$(,)?} => {$(
			$(
				#[deprecated = "deprecated - probably still supported, but discouraged (usually in favor of a better alternative)."]
				/// `deprecated`
				$(compile_error!($deprecated))?
			)?
			$(
				#[deprecated = "experimental - not for production code and likely not well supported yet."]
				/// `experimental`
				$(compile_error!($experimental))?
			)?
			$(
				#[deprecated = "obsolete - most likely removed from most browsers that used to support it."]
				/// `obsolete`
				$(compile_error!($obsolete))?
			)?
			#[allow(deprecated)]
			$(#[$($attribute_token)*])*
			pub trait $name<Aspect: ?Sized = Attribute>: Sealed {
				#[inline(always)]
				fn static_validate_on(_: Self) where Self: Sized {
					// Intentionally no operation.
				}
			}
			#[allow(deprecated)]
			impl dyn $name<Attribute> {
				#[must_use]
				pub const NAME: &'static str = heck_but_macros::stringify_kebab_case!($name);
			}
			#[allow(deprecated)]
			impl<T: Global<Attribute>> $name<dyn Global<Attribute>> for T {}
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

pub trait EventInfo: Sealed {
	const NAME: &'static str;
	type Bubbles: YesNo;
	type Cancelable: YesNo;
}

pub trait YesBubbles {
	const OK: () = ();
}
impl<T: ?Sized> YesBubbles for T where T: EventInfo<Bubbles = Yes> {}
pub trait NoBubbles {
	const OK: () = ();
}
impl<T: ?Sized> NoBubbles for T where T: EventInfo<Bubbles = No> {}

pub trait YesCancelable {
	const OK: () = ();
}
impl<T: ?Sized> YesCancelable for T where T: EventInfo<Cancelable = Yes> {}
pub trait NoCancelable {
	const OK: () = ();
}
impl<T: ?Sized> NoCancelable for T where T: EventInfo<Cancelable = No> {}

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
	use super::{EventInfo, Global, No, Sealed, Yes};
	#[allow(deprecated)]
	use crate::aspects::Event;

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
				#[deprecated = "deprecated - probably still supported, but discouraged (usually in favor of a better alternative)."]
				/// `deprecated`
				$(compile_error!($deprecated))?
			)?
			$(
				#[deprecated = "non-standard (not on a standards track) - likely not well supported or with incompatible implementations."]
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
			#[allow(deprecated)]
			pub trait $name<Aspect: ?Sized = Event>: Sealed {
				#[inline(always)]
				fn static_validate_on(_: Self) where Self: Sized {
					// Intentionally left empty.
				}
			}
			#[allow(deprecated)]
			impl EventInfo for dyn $name<Event> {
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
				impl<T: Global<Event>> $name<dyn Global<Event>> for T {}
				$(compile_error!($bubbles))?
			)?
			$(
				$($(
					impl $name<Event> for crate::$namespace::elements::$element {}
				)*)?
				$(
					#[allow(deprecated)]
					impl<T: Global<Event>> $name<dyn Global<Event>> for T {}
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
