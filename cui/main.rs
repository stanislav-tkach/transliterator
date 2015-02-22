extern crate translator;

fn main()
{
	let args: Vec<String> = std::env::args().collect();
	if 2 != args.len()
	{
		println!("Usage:");
		println!("{} <text to translate>", args[0]);
		return;
	}

	let translator = translator::Translator::new();
	println!("{}", translator.translate(args[1].as_slice()));
}
