#![no_std]
#![no_main]

use agb::{
  include_aseprite,
  display::{self, object::Graphics},
};

const PLAYER: &Graphics = include_aseprite!("gfx/test-sprite.aseprite");

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
  let object_mgr = gba.display.object.get_managed();

  let mut player = object_mgr.object_sprite(&PLAYER.sprites()[0]);
  player.set_x(50).set_y(50).show();

  object_mgr.commit();

  let mut px = 50i16;
  let mut dx = 1;
  let mut hflip = false;
	loop {
    px += dx;
    if px > 120 || px < 20 {
      hflip = !hflip;
      player.set_hflip(hflip);
      dx = -dx;
    }
    player.set_x(px as u16);
    display::busy_wait_for_vblank();
    object_mgr.commit();
  }
}
