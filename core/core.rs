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
      rules: HashMap::new(),
    }
  }

  pub fn default() -> Translator
  {
    Translator
    {
      rules: HashMap::new(),
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
