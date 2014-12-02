use std::collections::HashMap;

struct Translator
{
  rules: HashMap<(char, char), char>,
}

impl Translator
{
  fn new() -> Translator
  {
    Translator
    {
      rules: HashMap::new(),
    }
  }

  fn default() -> Translator
  {
    Translator
    {
      rules: HashMap::new(),
    }
  }

  fn translate_char(&self, current : char, previous : char) -> char
  {
    // TODO: FIXME.
    'a'
  }

  pub fn translate(&self, text : &str) -> String
  {
    let mut result = String::with_capacity(text.len() * std::mem::size_of::<char>());

    let mut previous = ' ';
    for c in text.chars()
    {
      previous = self.translate_char(c, previous);
      result.push(previous);
    }

    result
  }
}

pub fn test()
{
  println!("transliterator!")

  let mut test = HashMap::new();

  test.insert(('a', 'a'), 'a');
  test.insert(('a', 'b'), 'b');
}
