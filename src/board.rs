use rand::Rng;
use console_engine::{ConsoleEngine, MouseButton, pixel, Color, KeyCode};
use std::process::exit;
use crate::board::State::{Hidden, Revealed, Marked};


static MINE_CHANCE: f32 = 0.1;

#[derive(Debug, Clone, PartialEq)]
pub enum State
{
	Hidden,
	Revealed,
	Marked
}

pub struct Board
{
	pub state: Vec<Vec<State>>,
	pub mines: Vec<Vec<bool>>,
	width: i32,
	height: i32,
	difficulty: i32
}

impl Board
{
	pub fn init(w: i32, h: i32, d: i32) -> Board
	{
		let mut b = Board
		{
			state: vec![vec![Hidden; h as usize]; w as usize],
			mines: vec![vec![false; h as usize]; w as usize],
			width: w,
			height: h,
			difficulty: d
		};
		b.reset();
		return b;
	}

	fn reset(&mut self)
	{
		let mine_factor: f32 = self.difficulty as f32 / 5.0f32;
		for x in 0..self.width
		{
			for y in 0..self.height
			{
				self.state[x as usize][y as usize] = Hidden;

				let c: f32 = rand::thread_rng().gen();
				if c <= MINE_CHANCE * mine_factor
				{
					self.mines[x as usize][y as usize] = true;
				}
				else
				{
					self.mines[x as usize][y as usize] = false;
				}
			}
		}
	}

	pub fn draw(&self, engine: &mut ConsoleEngine)
	{
		for x in 0..self.width
		{
			for y in 0..self.height
			{
				match self.state[x as usize][y as usize]
				{
					State::Hidden => {
						engine.set_pxl(x, y, pixel::pxl_bg(' ', Color::Grey));
					}
					State::Marked => {
						engine.set_pxl(x, y, pixel::pxl_bg('X', Color::Green));
					}
					State::Revealed => {
						if self.mines[x as usize][y as usize]
						{
							engine.set_pxl(x, y, pixel::pxl_fg('X', Color::Red));
						}
						else
						{
							let nearbyMines = self.nearby_mines(x, y);
							engine.print(x, y, nearbyMines.to_string().as_str());
						}
					}
				}
			}
		}
	}

	fn nearby_mines(&self, x_pos: i32, y_pos: i32) -> i32
	{
		let x_min = 0.max(x_pos-1);
		let x_max = (self.width-1).min(x_pos+1);
		let y_min = 0.max(y_pos-1);
		let y_max = (self.height-1).min(y_pos+1);

		let mut mines: i32 = 0;
		for x in x_min..x_max+1
		{
			for y in y_min..y_max+1
			{
				if self.mines[x as usize][y as usize]
				{
					mines += 1;
				}
			}
		}
		return mines;
	}

	fn reveal_neighbors(&mut self, x_pos: i32, y_pos: i32)
	{
		self.state[x_pos as usize][y_pos as usize] = Revealed;

		let x_min = 0.max(x_pos-1);
		let x_max = (self.width-1).min(x_pos+1);
		let y_min = 0.max(y_pos-1);
		let y_max = (self.height-1).min(y_pos+1);

		for x in x_min..x_max+1
		{
			for y in y_min..y_max+1
			{
				if self.nearby_mines(x, y) == 0 && self.state[x as usize][y as usize] != Revealed
				{
					self.reveal_neighbors(x, y);
				}
				self.state[x as usize][y as usize] = Revealed;
			}
		}
	}

	fn win_condition(&self) -> bool
	{
		for x in 0..self.width
		{
			for y in 0..self.height
			{
				if self.state[x as usize][y as usize] == Hidden && !self.mines[x as usize][y as usize]
				{
					return false;
				}
			}
		}
		return true;
	}

	pub fn input(&mut self, engine: &mut ConsoleEngine)
	{
		if engine.is_key_pressed(KeyCode::Char('q')) || engine.is_key_pressed(KeyCode::Esc)
		{
			exit(0);
		}
		if engine.is_key_pressed(KeyCode::Char('r'))
		{
			self.reset();
			return;
		}

		let left_pos = engine.get_mouse_press(MouseButton::Left);
		let right_pos = engine.get_mouse_press(MouseButton::Right);

		if left_pos.is_none() && right_pos.is_none() { return; }

		let (x32, y32) = if left_pos.is_some() { left_pos.unwrap() } else { right_pos.unwrap() };
		let (x, y) = (x32 as i32, y32 as i32);

		if x >= self.width || y >= self.height { return; }

		// prioritize left click
		if left_pos.is_some()
		{
			if self.mines[x as usize][y as usize]
			{
				self.end(engine, false);
			}
			else
			{
				self.state[x as usize][y as usize] = Revealed;

				if self.nearby_mines(x, y) == 0
				{
					self.reveal_neighbors(x, y);
				}

				if self.win_condition()
				{
					self.end(engine, true);
				}
			}
		}
		else
		{
			match self.state[x as usize][y as usize]
			{
				Marked => {
					self.state[x as usize][y as usize] = Hidden;
				}
				Revealed => {
					return;
				}
				Hidden => {
					self.state[x as usize][y as usize] = Marked;
				}
			}
		}
	}

	fn end(&mut self, engine: &mut ConsoleEngine, success: bool)
	{
		engine.clear_screen();
		let text = if success { "YOU WON" } else { "GAME OVER" };
		engine.print(self.width/2 - (text.len() as i32)/2, self.height/2 - 1, text);
		let end_text = "Press Q to Quit or R to Restart";
		engine.print(self.width/2 - (end_text.len() as i32)/2, self.height/2,end_text );
		engine.draw();
		loop
		{
			engine.wait_frame();
			if engine.is_key_pressed(KeyCode::Char('q'))
			{
				exit(0);
			}
			if engine.is_key_pressed(KeyCode::Char('r'))
			{
				self.reset();
				break;
			}
		}
	}
}
