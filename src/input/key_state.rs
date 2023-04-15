#[derive(PartialEq, Eq)]
pub enum KeyState {
    Up,
    Pressed,
    Down,
    Released,
}

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
    pub shift: KeyState,
    pub ctrl: KeyState,
    pub alt: KeyState,
    pub esc: KeyState,
}

impl KeysState {
    pub fn new() -> Self {
        KeysState {
            a: KeyState::Up,
            b: KeyState::Up,
            c: KeyState::Up,
            d: KeyState::Up,
            e: KeyState::Up,
            f: KeyState::Up,
            g: KeyState::Up,
            h: KeyState::Up,
            i: KeyState::Up,
            j: KeyState::Up,
            k: KeyState::Up,
            l: KeyState::Up,
            m: KeyState::Up,
            n: KeyState::Up,
            o: KeyState::Up,
            p: KeyState::Up,
            q: KeyState::Up,
            r: KeyState::Up,
            s: KeyState::Up,
            t: KeyState::Up,
            u: KeyState::Up,
            v: KeyState::Up,
            w: KeyState::Up,
            x: KeyState::Up,
            up: KeyState::Up,
            down: KeyState::Up,
            left: KeyState::Up,
            right: KeyState::Up,
            _0: KeyState::Up,
            _1: KeyState::Up,
            _2: KeyState::Up,
            _3: KeyState::Up,
            _4: KeyState::Up,
            _5: KeyState::Up,
            _6: KeyState::Up,
            _7: KeyState::Up,
            _8: KeyState::Up,
            _9: KeyState::Up,
            space: KeyState::Up,
            enter: KeyState::Up,
            mouse_left: KeyState::Up,
            mouse_right: KeyState::Up,
            mouse_middle: KeyState::Up,
            shift: KeyState::Up,
            ctrl: KeyState::Up,
            alt: KeyState::Up,
            esc: KeyState::Up,
        }
    }

    fn get_key_state(key: &KeyState, is_down: bool) -> KeyState {
        if is_down && *key != KeyState::Down {
            KeyState::Pressed
        } else if is_down {
            KeyState::Down
        } else if *key != KeyState::Up {
            KeyState::Released
        } else {
            KeyState::Up
        }
    }

    pub fn set_key_state(&mut self, keycode: sdl2::keyboard::Keycode, is_down: bool) {
        use sdl2::keyboard::Keycode;

        match keycode {
            Keycode::A => self.a = Self::get_key_state(&self.a, is_down),
			Keycode::B => self.b = Self::get_key_state(&self.b, is_down),
			Keycode::C => self.c = Self::get_key_state(&self.c, is_down),
			Keycode::D => self.d = Self::get_key_state(&self.d, is_down),
			Keycode::E => self.e = Self::get_key_state(&self.e, is_down),
			Keycode::F => self.f = Self::get_key_state(&self.f, is_down),
			Keycode::G => self.g = Self::get_key_state(&self.g, is_down),
			Keycode::H => self.h = Self::get_key_state(&self.h, is_down),
			Keycode::I => self.i = Self::get_key_state(&self.i, is_down),
			Keycode::J => self.j = Self::get_key_state(&self.j, is_down),
			Keycode::K => self.k = Self::get_key_state(&self.k, is_down),
			Keycode::L => self.l = Self::get_key_state(&self.l, is_down),
			Keycode::M => self.m = Self::get_key_state(&self.m, is_down),
			Keycode::N => self.n = Self::get_key_state(&self.n, is_down),
			Keycode::O => self.o = Self::get_key_state(&self.o, is_down),
			Keycode::P => self.p = Self::get_key_state(&self.p, is_down),
			Keycode::Q => self.q = Self::get_key_state(&self.q, is_down),
			Keycode::R => self.r = Self::get_key_state(&self.r, is_down),
			Keycode::S => self.s = Self::get_key_state(&self.s, is_down),
			Keycode::T => self.t = Self::get_key_state(&self.t, is_down),
			Keycode::U => self.u = Self::get_key_state(&self.u, is_down),
			Keycode::V => self.v = Self::get_key_state(&self.v, is_down),
			Keycode::W => self.w = Self::get_key_state(&self.w, is_down),
			Keycode::X => self.x = Self::get_key_state(&self.x, is_down),
			Keycode::Escape => self.esc = Self::get_key_state(&self.esc, is_down),
            _ => {
				println!("Keycode: {:?}", keycode);
			}
        }
    }
}
