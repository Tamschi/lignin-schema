use lignin_schema::{
	events::{click, error},
	html::{
		attributes::{class, href},
		elements::{a, br},
	},
	HasContent, NoBubbles, NoCancelable, YesBubbles, YesCancelable,
};

pub fn validate() {
	class::static_validate_on(br);
	click::static_validate_on(br);

	HasContent::static_validate_on(a);
	class::static_validate_on(a);
	href::static_validate_on(a);
	click::static_validate_on(a);

	let _ = <dyn click as YesBubbles>::OK;
	let _ = <dyn click as YesCancelable>::OK;

	let _ = <dyn error as NoBubbles>::OK;
	let _ = <dyn error as NoCancelable>::OK;
}
