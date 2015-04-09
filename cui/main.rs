extern crate translator;

fn main()
{
	let args = std::env::args();
	if args.len() < 2
	{
		println!("Usage:");
		println!("{} <text to translate>", args.take(1).last().unwrap_or("transliterator".to_string()));
		return;
	}

	let text = args.skip(1).fold(String::new(), |result, param| result + " " + &param).trim_left().to_string();

	let translator = translator::Translator::new();
	println!("{}", translator.translate(&text));
}
