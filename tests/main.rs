extern crate core;

#[test]
fn empty()
{
	let translator = core::Translator::new();

	assert_eq!("", translator.translate(""));
	assert_eq!(" ", translator.translate(" "));
	assert_eq!("  ", translator.translate("  "));
	assert_eq!("   ", translator.translate("   "));
}

#[test]
fn letters()
{
	let translator = core::Translator::new();

	assert_eq!("а", translator.translate("a"));
	assert_eq!("б", translator.translate("b"));
	assert_eq!("в", translator.translate("v"));
	assert_eq!("г", translator.translate("g"));
	assert_eq!("д", translator.translate("d"));
	assert_eq!("е", translator.translate("e"));
	assert_eq!("ё", translator.translate("jo"));
	assert_eq!("ж", translator.translate("zh"));
	assert_eq!("з", translator.translate("z"));
	assert_eq!("и", translator.translate("i"));
	assert_eq!("й", translator.translate("j"));
	assert_eq!("к", translator.translate("k"));
	assert_eq!("л", translator.translate("l"));
	assert_eq!("м", translator.translate("m"));
	assert_eq!("н", translator.translate("n"));
	assert_eq!("о", translator.translate("o"));
	assert_eq!("п", translator.translate("p"));
	assert_eq!("р", translator.translate("r"));
	assert_eq!("с", translator.translate("s"));
	assert_eq!("т", translator.translate("t"));
	assert_eq!("у", translator.translate("u"));
	assert_eq!("ф", translator.translate("f"));
	assert_eq!("х", translator.translate("h"));
	assert_eq!("ц", translator.translate("c"));
	assert_eq!("ч", translator.translate("ch"));
	assert_eq!("ш", translator.translate("sh"));
	assert_eq!("щ", translator.translate("shh"));
	assert_eq!("ъ", translator.translate("#"));
	assert_eq!("ы", translator.translate("y"));
	assert_eq!("ь", translator.translate("'"));
	assert_eq!("э", translator.translate("je"));
	assert_eq!("ю", translator.translate("ju"));
	assert_eq!("я", translator.translate("ja"));
}

fn capital_letters()
{
	let translator = core::Translator::new();

	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
	assert_eq!("а", translator.translate(""));
}

#[test]
fn words()
{
}

