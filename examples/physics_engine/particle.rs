use pg_sdl::color::Colors;
use pg_sdl::vector2::Vec2;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct State {
	n: usize,
	n_c: usize,
	dt: f32,
	theta: Vec<f32>,
	v_theta: Vec<f32>,
	a_theta: Vec<f32>,
	p_x: Vec<f32>,
	v_x: Vec<f32>,
	a_x: Vec<f32>,
	p_y: Vec<f32>,
	v_y: Vec<f32>,
	a_y: Vec<f32>,
	r_x: Vec<f32>,
	r_y: Vec<f32>,
	r_t: Vec<f32>,
}

fn rk4solver(nb: u8, state: &mut State, delta: f32) -> bool {
	let m_initial_state = State {
		n: 0,
		n_c: 0,
		dt: 0.0,
		theta: Vec::new(),
		v_theta: Vec::new(),
		a_theta: Vec::new(),
		p_x: Vec::new(),
		v_x: Vec::new(),
		a_x: Vec::new(),
		p_y: Vec::new(),
		v_y: Vec::new(),
		a_y: Vec::new(),
		r_x: Vec::new(),
		r_y: Vec::new(),
		r_t: Vec::new(),
	};

	match nb {
		1 => {
			state.dt = 0.0;
		}
		2 => {}
		3 => {
			for i in 0..state.n {
				// (int i = 0; i < state.n; ++i)
				state.v_theta[i] = m_initial_state.v_theta[i] + delta * state.a_theta[i] / 2.0;
				state.theta[i] = m_initial_state.theta[i] + delta * state.v_theta[i] / 2.0;
				state.v_x[i] = m_initial_state.v_x[i] + delta * state.a_x[i] / 2.0;
				state.v_y[i] = m_initial_state.v_y[i] + delta * state.a_y[i] / 2.0;
				state.p_x[i] = m_initial_state.p_x[i] + delta * state.v_x[i] / 2.0;
				state.p_y[i] = m_initial_state.p_y[i] + delta * state.v_y[i] / 2.0;
			}
			state.dt = delta / 2.0;
		}
		4 => {
			for i in 0..state.n {
				// (int i = 0; i < state.n; ++i)
				state.v_theta[i] = m_initial_state.v_theta[i] + delta * state.a_theta[i];
				state.theta[i] = m_initial_state.theta[i] + delta * state.v_theta[i];
				state.v_x[i] = m_initial_state.v_x[i] + delta * state.a_x[i];
				state.v_y[i] = m_initial_state.v_y[i] + delta * state.a_y[i];
				state.p_x[i] = m_initial_state.p_x[i] + delta * state.v_x[i];
				state.p_y[i] = m_initial_state.p_y[i] + delta * state.v_y[i];
			}

			state.dt = delta;
		}

		_ => {}
	}

	let next_nb = nb + 1;

	return next_nb == 5;
}

fn rk4ode_solver(nb: u8, system: &mut State, delta: f32) {
	let mut stage_weight: f32 = 0.0;
	match nb {
		1 => stage_weight = 1.0,
		2 => stage_weight = 2.0,
		3 => stage_weight = 2.0,
		4 => stage_weight = 1.0,
		_ => stage_weight = 0.0,
	}

	let mut m_accumulator = State {
		n: 0,
		n_c: 0,
		dt: 0.0,
		theta: Vec::new(),
		v_theta: Vec::new(),
		a_theta: Vec::new(),
		p_x: Vec::new(),
		v_x: Vec::new(),
		a_x: Vec::new(),
		p_y: Vec::new(),
		v_y: Vec::new(),
		a_y: Vec::new(),
		r_x: Vec::new(),
		r_y: Vec::new(),
		r_t: Vec::new(),
	};

	for i in 0..system.n {
		// (int i = 0; i < system.n; ++i)
		m_accumulator.v_theta[i] += (delta / 6.0) * system.a_theta[i] * stage_weight;
		m_accumulator.theta[i] += (delta / 6.0) * system.v_theta[i] * stage_weight;
		m_accumulator.v_x[i] += (delta / 6.0) * system.a_x[i] * stage_weight;
		m_accumulator.v_y[i] += (delta / 6.0) * system.a_y[i] * stage_weight;
		m_accumulator.p_x[i] += (delta / 6.0) * system.v_x[i] * stage_weight;
		m_accumulator.p_y[i] += (delta / 6.0) * system.v_y[i] * stage_weight;
	}

	for i in 0..system.n_c {
		// (int i = 0; i < system.n_c; ++i) {
		m_accumulator.r_x[i] += (delta / 6.0) * system.r_x[i] * stage_weight;
		m_accumulator.r_y[i] += (delta / 6.0) * system.r_y[i] * stage_weight;
		m_accumulator.r_t[i] += (delta / 6.0) * system.r_t[i] * stage_weight;
	}

	if nb == 4 {
		for i in 0..system.n {
			// (int i = 0; i < system.n; ++i)
			system.v_theta[i] = m_accumulator.v_theta[i];
			system.theta[i] = m_accumulator.theta[i];
			system.v_x[i] = m_accumulator.v_x[i];
			system.v_y[i] = m_accumulator.v_y[i];
			system.p_x[i] = m_accumulator.p_x[i];
			system.p_y[i] = m_accumulator.p_y[i];
		}

		for i in 0..system.n_c {
			// (int i = 0; i < system.n_c; ++i)
			system.r_x[i] = m_accumulator.r_x[i];
			system.r_y[i] = m_accumulator.r_y[i];
			system.r_t[i] = m_accumulator.r_t[i];
		}
	}

	let nb = nb + 1; // next_nb
}

#[derive(Debug, Clone)]
/// A Particle is an object that have a mass, a position, a velocity, and respond to forces.
///
/// It has no shape, but it can be drawn as a circle.
///
/// It is the basic element of a physics engine.
pub struct Particle {
	mass: f32,
	position: Vec2,
	velocity: Vec2,
	force_accumulator: Vec<Vec2>,
	last_velocity: Vec2,
	last_acceleration: Vec2,
	radius: f32,
	color: Color,
}
impl Particle {
	pub fn new(mass: f32, position: Vec2, radius: f32, color: Color) -> Self {
		Self {
			mass,
			position,
			velocity: Vec2::ZERO,
			force_accumulator: Vec::new(),
			last_velocity: Vec2::ZERO,
			last_acceleration: Vec2::ZERO,
			radius,
			color,
		}
	}

	pub fn get_position(&self) -> Vec2 {
		self.position
	}
	pub fn set_position(&mut self, position: Vec2) {
		self.position = position;
	}

	pub fn get_velocity(&self) -> Vec2 {
		self.velocity
	}
	pub fn set_velocity(&mut self, velocity: Vec2) {
		self.velocity = velocity;
	}

	pub fn get_mass(&self) -> f32 {
		self.mass
	}

	pub fn apply_force(&mut self, force: Vec2) {
		self.force_accumulator.push(force);
	}
	pub fn get_force(&self) -> Vec2 {
		self.force_accumulator.clone().into_iter().sum()
	}

	pub fn collide_point(&self, point: Vec2) -> bool {
		(self.position - point).length() < self.radius
	}

	pub fn update(&mut self, delta: f32) {
		let force = self.get_force();
		let acceleration = force / self.mass;

		// Runge-Kutta 1st order (midpoint method)
		// let average_acceleration = (3.0 * self.last_acceleration - acceleration) / 2.0;
		// self.velocity += average_acceleration * delta;
		// let average_velocity = (self.last_velocity + self.velocity) / 2.0;
		// self.position += average_velocity * delta;

		// Euler
		//self.velocity += acceleration * delta;
		//self.position += self.velocity * delta;

		// Runge-Kutta 4nd order (RK4)

		let k1 = acceleration;
		let k2 = (self.last_acceleration + k1) / 2.0;
		let k3 = (self.last_acceleration + k2) / 2.0;
		let k4 = (self.last_acceleration + k3) / 2.0;
		let total_acceleration = (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;
		// println!("k1: {:?}, k2: {:?}, k3: {:?}, k4: {:?} total: {:?}", k1, k2, k3, k4, total_acceleration);
		self.velocity += total_acceleration * delta;
		let k1 = self.velocity;
		let k2 = (self.last_velocity + k1) / 2.0;
		let k3 = (self.last_velocity + k2) / 2.0;
		let k4 = (self.last_velocity + k3) / 2.0;
		let total_velocity = (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;
		self.position += total_velocity * delta;

		self.last_velocity = self.velocity;
		self.last_acceleration = acceleration;
	}

	pub fn clear_force_accumulator(&mut self) {
		self.force_accumulator.clear();
	}

	pub fn draw(&self, canvas: &mut Canvas<Window>) {
		if self.radius == 0.0 {
			return;
		}
		DrawRenderer::filled_circle(
			canvas,
			self.position.x as i16,
			self.position.y as i16,
			self.radius as i16,
			self.color,
		)
		.unwrap();
		DrawRenderer::circle(canvas, self.position.x as i16, self.position.y as i16, self.radius as i16, Colors::BLACK)
			.unwrap();
	}

	pub fn draw_forces(&self, canvas: &mut Canvas<Window>, scaler: f32) {
		if self.radius == 0.0 {
			return;
		}
		for (i, force) in self.force_accumulator.iter().enumerate() {
			let color = if i == self.force_accumulator.len() - 1 { Colors::VIOLET } else { Colors::LIGHT_YELLOW };
			draw_arrow(canvas, color, self.position, self.position + *force * scaler, 5.0);
		}
	}
}

/// Draw an arrow from start to end with the head at the end.
fn draw_arrow(canvas: &mut Canvas<Window>, color: Color, start: Vec2, end: Vec2, width: f32) {
	if start == end {
		return;
	}
	let x_dir = end - start;
	let y_dir = x_dir.perpendicular() * width / 2.0;
	let transform = |v: Vec2| start + v.linear_transform(x_dir, y_dir);

	let head_back: f32 = 1.0 - 3.0 * width / x_dir.length();

	let mut points = Vec::from([
		Vec2::new(head_back, -1.0),
		Vec2::new(head_back, -3.0),
		Vec2::new(1.0, 0.0),
		Vec2::new(head_back, 3.0),
		Vec2::new(head_back, 1.0),
	]);
	if x_dir.length() > 3.0 * width {
		points.append(&mut Vec::from([Vec2::new(0.0, 1.0), Vec2::new(0.0, -1.0)]));
	}
	points.iter_mut().for_each(|v| *v = transform(*v));
	let points_x: Vec<i16> = points.iter().map(|v| v.x as i16).collect();
	let points_y: Vec<i16> = points.iter().map(|v| v.y as i16).collect();

	DrawRenderer::filled_polygon(canvas, &points_x, &points_y, color).unwrap();
	DrawRenderer::polygon(canvas, &points_x, &points_y, Colors::BLACK).unwrap();
}
