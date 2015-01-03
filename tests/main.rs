extern crate core;

#[test]
fn main()
{
	let translator = core::Translator::new();

	assert_eq!("a", translator.translate("a").as_slice());
}
