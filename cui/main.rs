extern crate translator;

fn main()
{
	let args = std::env::args();
	if args.len() < 2
	{
		println!("Usage:");
		// TODO: Fix this ugly construction?
		println!("{} <text to translate>", args.take(1).last().unwrap());
		return;
	}

	let text = args.skip(1).fold(String::new(), |result, param| result + " " + &param);

	let translator = translator::Translator::new();
	println!("{}", translator.translate(&text));
}
