use sdl2::keyboard::Keycode;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum KeyState {
	Up,
	Pressed,
	Down,
	Released,
}

// TODO suggestion key state avec double press
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ChadKeyState {
	Up { released_time: std::time::Instant },
	Down,
	Released,
	Pressed,
	DoublePressed,
}

impl ChadKeyState {
	const TIME_TO_CONSECUTIVE_MS: u128 = 100;

	pub fn update(&mut self) {
		match self {
			Self::Pressed | Self::DoublePressed => {
				*self = Self::Down;
			}
			Self::Released => {
				*self = Self::Up { released_time: std::time::Instant::now() };
			}
			_ => {}
		};
	}

	pub fn press(&mut self) {
		*self = match self {
			Self::Up { released_time } => {
				if released_time.elapsed().as_millis() < Self::TIME_TO_CONSECUTIVE_MS {
					Self::DoublePressed
				} else {
					Self::Pressed
				}
			}
			_ => panic!("Cannot press a key that is not up or released ({:?})", self),
		};
	}

	pub fn release(&mut self) {
		*self = match self {
			Self::Down => Self::Released,
			_ => panic!("Cannot release a key that is not pressed or down (key is {:?})", self),
		};
	}

	pub fn is_up(&self) -> bool {
		match self {
			Self::Up { .. } => true,
			_ => false,
		}
	}
	pub fn is_pressed(&self) -> bool {
		*self == Self::Pressed
	}
	pub fn is_down(&self) -> bool {
		match self {
			Self::Down { .. } => true,
			_ => false,
		}
	}
	pub fn is_released(&self) -> bool {
		match self {
			Self::Released { .. } => true,
			_ => false,
		}
	}
	pub fn is_double_pressed(&self) -> bool {
		*self == Self::DoublePressed
	}
}

impl KeyState {
	pub fn new() -> Self {
		Self::Up
	}
	pub fn update(&mut self) {
		match self {
			Self::Pressed => {
				*self = Self::Down;
			}
			Self::Released => {
				*self = Self::Up;
			}
			_ => {}
		};
	}

	pub fn press(&mut self) {
		*self = Self::Pressed
	}
	pub fn release(&mut self) {
		*self = Self::Released
	}

	pub fn is_up(&self) -> bool {
		*self == Self::Up
	}
	pub fn is_pressed(&self) -> bool {
		*self == Self::Pressed
	}
	pub fn is_down(&self) -> bool {
		*self == Self::Down
	}
	pub fn is_released(&self) -> bool {
		*self == Self::Released
	}
}

#[derive(Debug)]
pub struct KeysState {
	pub a: KeyState,
	pub b: KeyState,
	pub c: KeyState,
	pub d: KeyState,
	pub e: KeyState,
	pub f: KeyState,
	pub g: KeyState,
	pub h: KeyState,
	pub i: KeyState,
	pub j: KeyState,
	pub k: KeyState,
	pub l: KeyState,
	pub m: KeyState,
	pub n: KeyState,
	pub o: KeyState,
	pub p: KeyState,
	pub q: KeyState,
	pub r: KeyState,
	pub s: KeyState,
	pub t: KeyState,
	pub u: KeyState,
	pub v: KeyState,
	pub w: KeyState,
	pub x: KeyState,
	pub y: KeyState,
	pub z: KeyState,
	pub up: KeyState,
	pub down: KeyState,
	pub left: KeyState,
	pub right: KeyState,
	pub _0: KeyState,
	pub _1: KeyState,
	pub _2: KeyState,
	pub _3: KeyState,
	pub _4: KeyState,
	pub _5: KeyState,
	pub _6: KeyState,
	pub _7: KeyState,
	pub _8: KeyState,
	pub _9: KeyState,
	pub space: KeyState,
	pub enter: KeyState,
	pub mouse_left: KeyState,
	pub mouse_right: KeyState,
	pub mouse_middle: KeyState,
	pub escape: KeyState,
	pub backspace: KeyState,
	pub lctrl: KeyState,
	pub rctrl: KeyState,
	pub tab: KeyState,
	pub lshift: KeyState,
	pub rshift: KeyState,
	pub lalt: KeyState,
	pub ralt: KeyState,
	pub lgui: KeyState,
	pub rgui: KeyState,
}

impl KeysState {
	pub fn new() -> Self {
		KeysState {
			a: KeyState::new(),
			b: KeyState::new(),
			c: KeyState::new(),
			d: KeyState::new(),
			e: KeyState::new(),
			f: KeyState::new(),
			g: KeyState::new(),
			h: KeyState::new(),
			i: KeyState::new(),
			j: KeyState::new(),
			k: KeyState::new(),
			l: KeyState::new(),
			m: KeyState::new(),
			n: KeyState::new(),
			o: KeyState::new(),
			p: KeyState::new(),
			q: KeyState::new(),
			r: KeyState::new(),
			s: KeyState::new(),
			t: KeyState::new(),
			u: KeyState::new(),
			v: KeyState::new(),
			w: KeyState::new(),
			x: KeyState::new(),
			y: KeyState::new(),
			z: KeyState::new(),
			up: KeyState::new(),
			down: KeyState::new(),
			left: KeyState::new(),
			right: KeyState::new(),
			_0: KeyState::new(),
			_1: KeyState::new(),
			_2: KeyState::new(),
			_3: KeyState::new(),
			_4: KeyState::new(),
			_5: KeyState::new(),
			_6: KeyState::new(),
			_7: KeyState::new(),
			_8: KeyState::new(),
			_9: KeyState::new(),
			space: KeyState::new(),
			enter: KeyState::new(),
			mouse_left: KeyState::new(),
			mouse_right: KeyState::new(),
			mouse_middle: KeyState::new(),
			escape: KeyState::new(),
			backspace: KeyState::new(),
			lctrl: KeyState::new(),
			rctrl: KeyState::new(),
			tab: KeyState::new(),
			lshift: KeyState::new(),
			rshift: KeyState::new(),
			lalt: KeyState::new(),
			ralt: KeyState::new(),
			lgui: KeyState::new(),
			rgui: KeyState::new(),
		}
	}

	pub fn get_key(&self, keycode: Keycode) -> &KeyState {
		match keycode {
			Keycode::Backspace => &self.backspace,
			Keycode::A => &self.a,
			Keycode::B => &self.b,
			Keycode::C => &self.c,
			Keycode::D => &self.d,
			Keycode::E => &self.e,
			Keycode::F => &self.f,
			Keycode::G => &self.g,
			Keycode::H => &self.h,
			Keycode::I => &self.i,
			Keycode::J => &self.j,
			Keycode::K => &self.k,
			Keycode::L => &self.l,
			Keycode::M => &self.m,
			Keycode::N => &self.n,
			Keycode::O => &self.o,
			Keycode::P => &self.p,
			Keycode::Q => &self.q,
			Keycode::R => &self.r,
			Keycode::S => &self.s,
			Keycode::T => &self.t,
			Keycode::U => &self.u,
			Keycode::V => &self.v,
			Keycode::W => &self.w,
			Keycode::X => &self.x,
			Keycode::Y => &self.y,
			Keycode::Z => &self.z,
			Keycode::Up => &self.up,
			Keycode::Down => &self.down,
			Keycode::Left => &self.left,
			Keycode::Right => &self.right,
			Keycode::Num0 => &self._0,
			Keycode::Num1 => &self._1,
			Keycode::Num2 => &self._2,
			Keycode::Num3 => &self._3,
			Keycode::Num4 => &self._4,
			Keycode::Num5 => &self._5,
			Keycode::Num6 => &self._6,
			Keycode::Num7 => &self._7,
			Keycode::Num8 => &self._8,
			Keycode::Num9 => &self._9,
			Keycode::Space => &self.space,
			Keycode::Return => &self.enter,
			Keycode::LShift => &self.lshift,
			Keycode::RShift => &self.rshift,
			Keycode::LCtrl => &self.lctrl,
			Keycode::RCtrl => &self.rctrl,
			Keycode::LAlt => &self.lalt,
			Keycode::RAlt => &self.ralt,
			Keycode::Escape => &self.escape,
			Keycode::Tab => &self.tab,
			Keycode::LGui => &self.lgui,
			Keycode::RGui => &self.rgui,
			_ => todo!("Keycode {:?} not implemented", keycode),
		}
	}

	fn get_key_mut(&mut self, keycode: Keycode) -> &mut KeyState {
		match keycode {
			Keycode::Backspace => &mut self.backspace,
			Keycode::A => &mut self.a,
			Keycode::B => &mut self.b,
			Keycode::C => &mut self.c,
			Keycode::D => &mut self.d,
			Keycode::E => &mut self.e,
			Keycode::F => &mut self.f,
			Keycode::G => &mut self.g,
			Keycode::H => &mut self.h,
			Keycode::I => &mut self.i,
			Keycode::J => &mut self.j,
			Keycode::K => &mut self.k,
			Keycode::L => &mut self.l,
			Keycode::M => &mut self.m,
			Keycode::N => &mut self.n,
			Keycode::O => &mut self.o,
			Keycode::P => &mut self.p,
			Keycode::Q => &mut self.q,
			Keycode::R => &mut self.r,
			Keycode::S => &mut self.s,
			Keycode::T => &mut self.t,
			Keycode::U => &mut self.u,
			Keycode::V => &mut self.v,
			Keycode::W => &mut self.w,
			Keycode::X => &mut self.x,
			Keycode::Y => &mut self.y,
			Keycode::Z => &mut self.z,
			Keycode::Escape => &mut self.escape,
			Keycode::Up => &mut self.up,
			Keycode::Down => &mut self.down,
			Keycode::Left => &mut self.left,
			Keycode::Right => &mut self.right,
			Keycode::Num0 => &mut self._0,
			Keycode::Num1 => &mut self._1,
			Keycode::Num2 => &mut self._2,
			Keycode::Num3 => &mut self._3,
			Keycode::Num4 => &mut self._4,
			Keycode::Num5 => &mut self._5,
			Keycode::Num6 => &mut self._6,
			Keycode::Num7 => &mut self._7,
			Keycode::Num8 => &mut self._8,
			Keycode::Num9 => &mut self._9,
			Keycode::Space => &mut self.space,
			Keycode::Return => &mut self.enter,
			Keycode::LCtrl => &mut self.lctrl,
			Keycode::RCtrl => &mut self.rctrl,
			Keycode::LShift => &mut self.lshift,
			Keycode::RShift => &mut self.rshift,
			Keycode::LAlt => &mut self.lalt,
			Keycode::RAlt => &mut self.ralt,
			Keycode::Tab => &mut self.tab,
			Keycode::LGui => &mut self.lgui,
			Keycode::RGui => &mut self.rgui,
			_ => todo!("Keycode {:?} not implemented", keycode),
		}
	}

	pub fn press_key(&mut self, keycode: Keycode) {
		self.get_key_mut(keycode).press();
	}

	pub fn release_key(&mut self, keycode: Keycode) {
		self.get_key_mut(keycode).release();
	}

	pub fn as_mut_array(&mut self) -> [&mut KeyState; 56] {
		[
			&mut self.a,
			&mut self.b,
			&mut self.c,
			&mut self.d,
			&mut self.e,
			&mut self.f,
			&mut self.g,
			&mut self.h,
			&mut self.i,
			&mut self.j,
			&mut self.k,
			&mut self.l,
			&mut self.m,
			&mut self.n,
			&mut self.o,
			&mut self.p,
			&mut self.q,
			&mut self.r,
			&mut self.s,
			&mut self.t,
			&mut self.u,
			&mut self.v,
			&mut self.w,
			&mut self.x,
			&mut self.y,
			&mut self.z,
			&mut self.up,
			&mut self.down,
			&mut self.left,
			&mut self.right,
			&mut self.space,
			&mut self.enter,
			&mut self.mouse_left,
			&mut self.mouse_right,
			&mut self.mouse_middle,
			&mut self.escape,
			&mut self.backspace,
			&mut self._0,
			&mut self._1,
			&mut self._2,
			&mut self._3,
			&mut self._4,
			&mut self._5,
			&mut self._6,
			&mut self._7,
			&mut self._8,
			&mut self._9,
			&mut self.ralt,
			&mut self.lalt,
			&mut self.rctrl,
			&mut self.lctrl,
			&mut self.rshift,
			&mut self.lshift,
			&mut self.tab,
			&mut self.lgui,
			&mut self.rgui,
		]
	}

	pub fn shortcut_pressed(&self, shortcut: &Shortcut) -> bool {
		self.get_key(shortcut.key).is_pressed()
			&& shortcut.ctrl_keys.iter().all(|keys| keys.iter().any(|key| self.get_key(*key).is_down()))
	}
}

pub struct Shortcut {
	ctrl_keys: Vec<Vec<Keycode>>,
	key: Keycode,
}

impl Shortcut {
	pub fn new(ctrl_keys: Vec<Vec<Keycode>>, key: Keycode) -> Self {
		Self { ctrl_keys, key }
	}

	#[allow(non_snake_case)]
	pub fn COPY() -> Self {
		Self::new(vec![vec![Keycode::LCtrl, Keycode::RCtrl]], Keycode::C)
	}

	#[allow(non_snake_case)]
	pub fn PASTE() -> Self {
		Self::new(vec![vec![Keycode::LCtrl, Keycode::RCtrl]], Keycode::V)
	}

	#[allow(non_snake_case)]
	pub fn CUT() -> Self {
		Self::new(vec![vec![Keycode::LCtrl, Keycode::RCtrl]], Keycode::X)
	}
}
