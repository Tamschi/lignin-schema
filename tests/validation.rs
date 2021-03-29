use lignin_schema::{
	events::click,
	html::{
		attributes::{class, href},
		elements::{a, br},
	},
	Empty, HasContent,
};

pub fn validate() {
	br::static_validate(Empty);
	br::static_validate(class);
	br::static_validate(click);

	a::static_validate(Empty);
	a::static_validate(HasContent);
	a::static_validate(class);
	a::static_validate(href);
	a::static_validate(click);
}
