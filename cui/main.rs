extern crate core;

fn main()
{
  let args = std::os::args();
  if 2 != args.len()
  {
    println!("Usage:");
    println!("{} <text to translate>", args[0]);
    return;
  }

  let translator = core::Translator::new();
  println!("{}", translator.translate(args[1].as_slice()));
}
