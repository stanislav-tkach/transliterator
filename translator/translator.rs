use std::collections::HashMap;

pub struct Translator
{
	rules: HashMap<(char, char), char>,
}

impl Translator
{
	pub fn new() -> Translator
	{
		Translator
		{
			rules: get_default_rules(),
		}
	}

	// Returns translated characted and boolean value indicating that previous character should be erased.
	fn translate_char(&self, current : char, previous : char) -> (char, bool)
	{
		match self.rules.get(&(current, previous))
		{
			Some(result) => (*result, previous != ' '),
			None => match self.rules.get(&(current, ' '))
			{
				Some(result) => (*result, false),
				None => (current, false),
			}
		}
	}

	pub fn translate(&self, text : &str) -> String
	{
		let mut result = String::with_capacity(text.len());

		let mut previous = ' ';
		for c in text.chars()
		{
			let (translated, erase_previous) = self.translate_char(c, previous);
			if erase_previous
			{
				result.pop();
			}

			result.push(translated);
			previous = translated;
		}

		result
	}
}

fn get_default_rules() -> HashMap<(char, char), char>
{
	let mut result = HashMap::with_capacity(50);

	result.insert(('a', ' '), 'а');
	result.insert(('b', ' '), 'б');
	result.insert(('v', ' '), 'в');
	result.insert(('g', ' '), 'г');
	result.insert(('d', ' '), 'д');
	result.insert(('e', ' '), 'е');
	result.insert(('o', 'j'), 'ё');
	result.insert(('o', 'й'), 'ё');
	result.insert(('h', 'z'), 'ж');
	result.insert(('h', 'з'), 'ж');
	result.insert(('z', ' '), 'з');
	result.insert(('i', ' '), 'и');
	result.insert(('j', ' '), 'й');
	result.insert(('k', ' '), 'к');
	result.insert(('l', ' '), 'л');
	result.insert(('m', ' '), 'м');
	result.insert(('n', ' '), 'н');
	result.insert(('o', ' '), 'о');
	result.insert(('p', ' '), 'п');
	result.insert(('r', ' '), 'р');
	result.insert(('s', ' '), 'с');
	result.insert(('t', ' '), 'т');
	result.insert(('u', ' '), 'у');
	result.insert(('f', ' '), 'ф');
	result.insert(('h', ' '), 'х');
	result.insert(('x', ' '), 'х');
	result.insert(('c', ' '), 'ц');
	result.insert(('h', 'c'), 'ч');
	result.insert(('h', 'ц'), 'ч');
	result.insert(('h', 'h'), 'ш');
	result.insert(('h', 'с'), 'ш');
	result.insert(('h', 'ш'), 'щ');
	result.insert(('w', ' '), 'щ');
	result.insert(('#', ' '), 'ъ');
	result.insert(('y', ' '), 'ы');
	result.insert(('\'', ' '), 'ь');
	result.insert(('e', 'j'), 'э');
	result.insert(('e', 'й'), 'э');
	result.insert(('u', 'j'), 'ю');
	result.insert(('u', 'й'), 'ю');
	result.insert(('a', 'j'), 'я');
	result.insert(('a', 'й'), 'я');

	result.insert(('A', ' '), 'А');
	result.insert(('B', ' '), 'Б');
	result.insert(('V', ' '), 'В');
	result.insert(('G', ' '), 'Г');
	result.insert(('D', ' '), 'Д');
	result.insert(('E', ' '), 'Е');
	result.insert(('O', 'J'), 'Ё');
	result.insert(('O', 'Й'), 'Ё');
	result.insert(('H', 'Z'), 'Ж');
	result.insert(('H', 'З'), 'Ж');
	result.insert(('Z', ' '), 'З');
	result.insert(('I', ' '), 'И');
	result.insert(('J', ' '), 'Й');
	result.insert(('K', ' '), 'К');
	result.insert(('L', ' '), 'Л');
	result.insert(('M', ' '), 'М');
	result.insert(('N', ' '), 'Н');
	result.insert(('O', ' '), 'О');
	result.insert(('P', ' '), 'П');
	result.insert(('R', ' '), 'Р');
	result.insert(('S', ' '), 'С');
	result.insert(('T', ' '), 'Т');
	result.insert(('U', ' '), 'У');
	result.insert(('F', ' '), 'Ф');
	result.insert(('H', ' '), 'Х');
	result.insert(('X', ' '), 'Х');
	result.insert(('C', ' '), 'Ц');
	result.insert(('H', 'C'), 'Ч');
	result.insert(('H', 'Ц'), 'Ч');
	result.insert(('H', 'H'), 'Ш');
	result.insert(('H', 'С'), 'Ш');
	result.insert(('H', 'Ш'), 'Щ');
	result.insert(('W', ' '), 'Щ');
	result.insert(('#', 'ъ'), 'Ъ');
	result.insert(('Y', ' '), 'Ы');
	result.insert(('\'', 'ь'), 'Ь');
	result.insert(('E', 'J'), 'Э');
	result.insert(('E', 'Й'), 'Э');
	result.insert(('U', 'J'), 'Ю');
	result.insert(('U', 'Й'), 'Ю');
	result.insert(('A', 'J'), 'Я');
	result.insert(('A', 'Й'), 'Я');

	result
}

#[test]
fn translator_new() {
    let translator = Translator::new();
    assert_eq(Translator.rules, get_default_rules());
}
