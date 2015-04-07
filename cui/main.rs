extern crate translator;

fn main()
{
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2
	{
		println!("Usage:");
		println!("{} <text to translate>", args[0]);
		return;
	}

	let translator = translator::Translator::new();
	println!("{}", translator.translate(&args[1]));
}
