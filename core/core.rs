use std::collections::HashMap;

pub fn translate_char(current : char, previous : char) -> char
{
  // TODO: FIXME.
  'a'
}

pub fn translate(text : &str) -> String
{
  let mut result = String::with_capacity(text.len() * std::mem::size_of::<char>());

  let mut previous = ' ';
  for c in text.chars()
  {
    previous = translate_char(c, previous);
    result.push(previous);
  }

  result
}

pub fn test()
{
  println!("transliterator!")

  let mut test = HashMap::new();

  test.insert(('a', 'a'), 'a');
  test.insert(('a', 'b'), 'b');
}
