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

	assert_eq!("А", translator.translate(""));
	assert_eq!("Б", translator.translate(""));
	assert_eq!("В", translator.translate(""));
	assert_eq!("Г", translator.translate(""));
	assert_eq!("Д", translator.translate(""));
	assert_eq!("Е", translator.translate(""));
	assert_eq!("Ё", translator.translate(""));
	assert_eq!("Ж", translator.translate(""));
	assert_eq!("З", translator.translate(""));
	assert_eq!("И", translator.translate(""));
	assert_eq!("Й", translator.translate(""));
	assert_eq!("К", translator.translate(""));
	assert_eq!("Л", translator.translate(""));
	assert_eq!("М", translator.translate(""));
	assert_eq!("Н", translator.translate(""));
	assert_eq!("О", translator.translate(""));
	assert_eq!("П", translator.translate(""));
	assert_eq!("Р", translator.translate(""));
	assert_eq!("С", translator.translate(""));
	assert_eq!("Т", translator.translate(""));
	assert_eq!("У", translator.translate(""));
	assert_eq!("Ф", translator.translate(""));
	assert_eq!("Х", translator.translate(""));
	assert_eq!("Ц", translator.translate(""));
	assert_eq!("Ч", translator.translate(""));
	assert_eq!("Ш", translator.translate(""));
	assert_eq!("Щ", translator.translate(""));
	assert_eq!("Ъ", translator.translate(""));
	assert_eq!("Ы", translator.translate(""));
	assert_eq!("Ь", translator.translate(""));
	assert_eq!("Э", translator.translate(""));
	assert_eq!("Ю", translator.translate(""));
	assert_eq!("Я", translator.translate(""));
}

#[test]
fn words()
{
}

