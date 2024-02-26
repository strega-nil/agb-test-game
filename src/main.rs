#![no_std]
#![no_main]

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    agb::no_game(gba);
}
