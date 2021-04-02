#![allow(non_snake_case)]
mod board;
use std::str::FromStr;

extern crate clap;
use clap::{App, Arg, AppSettings};
use std::process::exit;


fn main()
{
	let matches = App::new("MineRuster")
		.setting(AppSettings::DisableVersion)
		.arg(Arg::with_name("width")
			.short("w")
			.long("width")
			.value_name("WIDTH")
			.help("Sets board width"))
		.arg(Arg::with_name("height")
			.short("h")
			.long("height")
			.value_name("HEIGHT")
			.help("Sets board height"))
		.arg(Arg::with_name("difficulty")
			.short("d")
			.long("difficulty")
			.value_name("DIFF")
			.help("Sets game difficulty"))
		.get_matches();

	let mut difficulty: i32 = 5;  // range 1-10
	let mut width: i32 = 60;
	let mut height: i32 = 30;

	if let Some(w) = matches.value_of("width")
	{
		width = FromStr::from_str(w).unwrap();
	}
	if let Some(h) = matches.value_of("height")
	{
		height = FromStr::from_str(h).unwrap();
	}
	if let Some(d) = matches.value_of("difficulty")
	{
		difficulty = FromStr::from_str(d).unwrap();
	}

	if difficulty < 1 || difficulty > 10
	{
		println!("Difficulty must be in range [1, 10]");
		exit(1);
	}

	let mut engine = console_engine::ConsoleEngine::init(width as u32, height as u32, 10);
	let mut b = board::Board::init(width, height, difficulty);

	loop
	{
		engine.wait_frame();
		engine.clear_screen();

		b.input(&mut engine);
		b.draw(&mut engine);

		engine.draw();
	}
}
