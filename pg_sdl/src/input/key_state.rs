use sdl2::keyboard::Keycode;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum KeyState { Up, Pressed, Down, Released }

impl KeyState {
    pub fn new() -> Self { Self::Up }
    pub fn update(&mut self) {
        match self {
            Self::Pressed => { *self = Self::Down; }
            Self::Released => { *self = Self::Up; }
            _ => {}
        };
    }

    pub fn press(&mut self) { *self = Self::Pressed }
    pub fn release(&mut self) { *self = Self::Released }

    pub fn is_up(&self) -> bool { *self == Self::Up }
    pub fn is_pressed(&self) -> bool { *self == Self::Pressed }
    pub fn is_down(&self) -> bool { *self == Self::Down }
    pub fn is_released(&self) -> bool { *self == Self::Released }
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
    pub escape: KeyState,
    pub backspace: KeyState,
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
            shift: KeyState::new(),
            ctrl: KeyState::new(),
            alt: KeyState::new(),
            escape: KeyState::new(),
            backspace: KeyState::new(),
        }
    }

    fn get_key(&mut self, keycode: Keycode) -> &mut KeyState {
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
            
            Keycode::LCtrl => &mut self.space,
            Keycode::RCtrl => &mut self.space,
            _ => todo!("mettre toutes les keys"),
        }
    }

    pub fn press_key(&mut self, keycode: Keycode) { self.get_key(keycode).press(); }
    pub fn release_key(&mut self, keycode: Keycode) { self.get_key(keycode).release(); }

    pub fn as_mut_array(&mut self) -> [&mut KeyState; 48] {
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
            &mut self.up,
            &mut self.down,
            &mut self.left,
            &mut self.right,
            &mut self.space,
            &mut self.enter,
            &mut self.mouse_left,
            &mut self.mouse_right,
            &mut self.mouse_middle,
            &mut self.shift,
            &mut self.ctrl,
            &mut self.alt,
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
        ]
    }
}
