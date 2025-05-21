use raylib::prelude::*;

mod rendersystem;
mod timer;

use rendersystem::Viewport;
use timer::Timer;

const TIMEOUT: f32 = 1.0;

fn main() {
	let mut viewport: Viewport = Viewport::init("Hide Cursor Timer Demo [Mechinsam]", 1280, 720, 100);

	let mut timer: Timer = Timer::new();
	timer.set(TIMEOUT);

	while !viewport.window.window_should_close() {
		if viewport.window.is_key_pressed(KeyboardKey::KEY_Q) {
			break;
		}

		if viewport.window.get_mouse_delta().x != 0f32 || viewport.window.get_mouse_delta().y != 0f32 {
			viewport.window.show_cursor();
			timer.set(TIMEOUT);
		}

		if timer.is_done() {
			viewport.window.hide_cursor();
		}

		let delta_time: f32 = viewport.window.get_frame_time();
		let mut drawer = viewport.window.begin_drawing(&viewport.thread);

		timer.tick(delta_time);
	}
}
