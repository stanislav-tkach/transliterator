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

#[test]
fn capital_letters()
{
	let translator = core::Translator::new();

	assert_eq!("А", translator.translate("A"));
	assert_eq!("Б", translator.translate("B"));
	assert_eq!("В", translator.translate("V"));
	assert_eq!("Г", translator.translate("G"));
	assert_eq!("Д", translator.translate("D"));
	assert_eq!("Е", translator.translate("E"));
	assert_eq!("Ё", translator.translate("JO"));
	assert_eq!("Ж", translator.translate("ZH"));
	assert_eq!("З", translator.translate("Z"));
	assert_eq!("И", translator.translate("I"));
	assert_eq!("Й", translator.translate("J"));
	assert_eq!("К", translator.translate("K"));
	assert_eq!("Л", translator.translate("L"));
	assert_eq!("М", translator.translate("M"));
	assert_eq!("Н", translator.translate("N"));
	assert_eq!("О", translator.translate("O"));
	assert_eq!("П", translator.translate("P"));
	assert_eq!("Р", translator.translate("R"));
	assert_eq!("С", translator.translate("S"));
	assert_eq!("Т", translator.translate("T"));
	assert_eq!("У", translator.translate("U"));
	assert_eq!("Ф", translator.translate("F"));
	assert_eq!("Х", translator.translate("X"));
	assert_eq!("Х", translator.translate("H")); // TODO !!!
	assert_eq!("Ц", translator.translate("C"));
	assert_eq!("Ч", translator.translate("CH"));
	assert_eq!("Ш", translator.translate("SH"));
	assert_eq!("Щ", translator.translate("SHH"));
	assert_eq!("Ъ", translator.translate("##"));
	assert_eq!("Ы", translator.translate("Y"));
	assert_eq!("Ь", translator.translate("''"));
	assert_eq!("Э", translator.translate("JE"));
	assert_eq!("Ю", translator.translate("JU"));
	assert_eq!("Я", translator.translate("JA"));
}

#[test]
fn words()
{
}

