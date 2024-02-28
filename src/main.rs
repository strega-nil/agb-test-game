#![no_std]
#![no_main]

use agb::{
	display::{self, object},
	include_aseprite,
	input::{self, Button},
};

const GRAPHICS: &object::Graphics = include_aseprite!("gfx/pong.aseprite");
const PADDLE_END: &object::Tag = GRAPHICS.tags().get("Paddle End");
const PADDLE_MID: &object::Tag = GRAPHICS.tags().get("Paddle Mid");
const BALL: &object::Tag = GRAPHICS.tags().get("Ball");

struct Paddle<'a> {
	top: object::Object<'a>,
	middle: object::Object<'a>,
	bottom: object::Object<'a>,

	pos_y: i32,
}

impl<'a> Paddle<'a> {
	const WIDTH: i32 = 16;
	const HEIGHT: i32 = 48;

	fn new(mgr: &'a object::OamManaged<'a>, pos_x: i32) -> Paddle<'a> {
		let mut res = Paddle {
			top: mgr.object_sprite(PADDLE_END.sprite(0)),
			middle: mgr.object_sprite(PADDLE_MID.sprite(0)),
			bottom: mgr.object_sprite(PADDLE_END.sprite(0)),

			pos_y: (display::HEIGHT - Paddle::HEIGHT) / 2,
		};
		res.mov(0);
		res.top.set_x(pos_x as u16).show();
		res.middle.set_x(pos_x as u16).show();
		res.bottom.set_x(pos_x as u16).set_vflip(true).show();

		res
	}

	fn mov(&mut self, d: i32) {
		self.pos_y = (self.pos_y + d).clamp(0, display::HEIGHT - Paddle::HEIGHT);
		self.top.set_y(self.pos_y as u16);
		self.middle.set_y((self.pos_y + 16) as u16);
		self.bottom.set_y((self.pos_y + 32) as u16);
	}
}

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
	let object_mgr = gba.display.object.get_managed();
	let mut input = input::ButtonController::new();

	const BALL_SIZE: i32 = 16;
	let mut ball_x = (display::WIDTH - BALL_SIZE) / 2;
	let mut ball_y = (display::WIDTH - BALL_SIZE) / 2;
	let mut ball_dx = 1;
	let mut ball_dy = 1;

	let mut ball = object_mgr.object_sprite(BALL.sprite(0));
	ball.set_x(ball_x as u16).set_y(ball_y as u16).show();

	let mut paddle_1 = Paddle::new(&object_mgr, 0);
	let mut paddle_2 = Paddle::new(&object_mgr, display::WIDTH - Paddle::WIDTH);

	loop {
		let d1 = input.y_tri() as i32;
		let d2 = input.is_pressed(Button::B) as i32 - input.is_pressed(Button::A) as i32;

		paddle_1.mov(d1);
		paddle_2.mov(d2);

		display::busy_wait_for_vblank();
		object_mgr.commit();
		input.update();
	}
}
