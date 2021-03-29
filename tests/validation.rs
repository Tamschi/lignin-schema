use lignin_schema::{
	events::click,
	html::{
		attributes::{class, href},
		elements::{a, br},
	},
	HasContent,
};

pub fn validate() {
	class::static_validate_on(br);
	click::static_validate_on(br);

	HasContent::static_validate_on(a);
	class::static_validate_on(a);
	href::static_validate_on(a);
	click::static_validate_on(a);
}
