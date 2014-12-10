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

	result
}
