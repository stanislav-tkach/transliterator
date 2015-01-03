extern crate core;

#[test]
fn characters()
{
	let translator = core::Translator::new();

	assert_eq!("а", translator.translate("a"));
}

#[test]
fn words()
{
}

