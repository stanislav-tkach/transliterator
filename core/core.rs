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

	fn translate_char(&self, current : char, previous : char) -> char
	{
		match self.rules.get(&(current, previous))
		{
			Some(result) => *result,
			None => match self.rules.get(&(current, ' '))
			{
				Some(result) => *result,
				None => current,
			}
		}
	}

	pub fn translate(&self, text : &str) -> String
	{
		let mut result = String::with_capacity(text.len());

		let mut previous = ' ';
		for c in text.chars()
		{
			previous = self.translate_char(c, previous);
			result.push(previous);
		}

		result
	}
}

fn get_default_rules() -> HashMap<(char, char), char>
{
	let mut result = HashMap::with_capacity(50);

	result.insert(('A', ' '), 'А');
	result.insert(('a', ' '), 'а');
	result.insert(('B', ' '), 'Б');
	result.insert(('b', ' '), 'б');
	result.insert(('V', ' '), 'В');
	result.insert(('v', ' '), 'в');
	result.insert(('G', ' '), 'Г');
	result.insert(('g', ' '), 'г');
	result.insert(('D', ' '), 'Д');
	result.insert(('d', ' '), 'д');
	result.insert(('E', ' '), 'Е');
	result.insert(('e', ' '), 'е');
	result.insert(('O', 'J'), 'Ё');
	result.insert(('O', 'Й'), 'Ё');
	result.insert(('o', 'j'), 'ё');
	result.insert(('o', 'й'), 'ё');
	result.insert(('H', 'Z'), 'Ж');
	result.insert(('H', 'З'), 'Ж');
	result.insert(('h', 'z'), 'ж');
	result.insert(('h', 'з'), 'ж');
	result.insert(('Z', ' '), 'З');
	result.insert(('z', ' '), 'з');
	result.insert(('I', ' '), 'И');
	result.insert(('i', ' '), 'и');
	result.insert(('J', ' '), 'Й');
	result.insert(('j', ' '), 'й');
	result.insert(('K', ' '), 'К');
	result.insert(('k', ' '), 'к');
	result.insert(('L', ' '), 'Л');
	result.insert(('l', ' '), 'л');
	result.insert(('M', ' '), 'М');
	result.insert(('m', ' '), 'м');
	result.insert(('N', ' '), 'Н');
	result.insert(('n', ' '), 'н');
	result.insert(('O', ' '), 'О');
	result.insert(('o', ' '), 'о');
	result.insert(('P', ' '), 'П');
	result.insert(('p', ' '), 'п');
	result.insert(('R', ' '), 'Р');
	result.insert(('r', ' '), 'р');
	result.insert(('S', ' '), 'С');
	result.insert(('s', ' '), 'с');
	result.insert(('T', ' '), 'Т');
	result.insert(('t', ' '), 'т');
	result.insert(('U', ' '), 'У');
	result.insert(('u', ' '), 'у');
	result.insert(('F', ' '), 'Ф');
	result.insert(('f', ' '), 'ф');
	result.insert(('H', ' '), 'Х');
	result.insert(('X', ' '), 'Х');
	result.insert(('h', ' '), 'х');
	result.insert(('x', ' '), 'х');
	result.insert(('C', ' '), 'Ц');
	result.insert(('c', ' '), 'ц');
	result.insert(('H', 'C'), 'Ч');
	result.insert(('H', 'Ц'), 'Ч');
	result.insert(('h', 'c'), 'ч');
	result.insert(('h', 'ц'), 'ч');
	result.insert(('H', 'H'), 'Ш');
	result.insert(('H', 'С'), 'Ш');
	result.insert(('h', 'h'), 'ш');
	result.insert(('h', 'с'), 'ш');
	result.insert(('H', 'Ш'), 'Щ');
	result.insert(('W', ' '), 'Щ');
	result.insert(('h', 'ш'), 'щ');
	result.insert(('w', ' '), 'щ');
	result.insert(('#', 'ъ'), 'Ъ');
	result.insert(('#', ' '), 'ъ');
	result.insert(('Y', ' '), 'Ы');
	result.insert(('y', ' '), 'ы');
	result.insert(('\'', 'ь'), 'Ь');
	result.insert(('\'', ' '), 'ь');
	result.insert(('E', 'J'), 'Э');
	result.insert(('E', 'Й'), 'Э');
	result.insert(('e', 'j'), 'э');
	result.insert(('e', 'й'), 'э');
	result.insert(('U', 'J'), 'Ю');
	result.insert(('U', 'Й'), 'Ю');
	result.insert(('u', 'j'), 'ю');
	result.insert(('u', 'й'), 'ю');
	result.insert(('A', 'J'), 'Я');
	result.insert(('A', 'Й'), 'Я');
	result.insert(('a', 'j'), 'я');
	result.insert(('a', 'й'), 'я');

	result
}
