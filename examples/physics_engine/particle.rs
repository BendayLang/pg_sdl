use nalgebra::{Matrix2, Matrix3, Point2, Similarity2, Transform2, Vector2};
use pg_sdl::camera::Camera;
use pg_sdl::color::Colors;
use pg_sdl::vector2::Vector2Plus;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct State {
	n: usize,
	n_c: usize,
	dt: f64,
	theta: Vector2<f64>,
	v_theta: Vector2<f64>,
	a_theta: Vector2<f64>,
	p_x: Vector2<f64>,
	v_x: Vector2<f64>,
	a_x: Vector2<f64>,
	p_y: Vector2<f64>,
	v_y: Vector2<f64>,
	a_y: Vector2<f64>,
	r_x: Vector2<f64>,
	r_y: Vector2<f64>,
	r_t: Vector2<f64>,
}

fn rk4solver(nb: u8, state: &mut State, delta: f64) -> bool {
	let m_initial_state = State {
		n: 0,
		n_c: 0,
		dt: 0.0,
		theta: Vector2::zeros(),
		v_theta: Vector2::zeros(),
		a_theta: Vector2::zeros(),
		p_x: Vector2::zeros(),
		v_x: Vector2::zeros(),
		a_x: Vector2::zeros(),
		p_y: Vector2::zeros(),
		v_y: Vector2::zeros(),
		a_y: Vector2::zeros(),
		r_x: Vector2::zeros(),
		r_y: Vector2::zeros(),
		r_t: Vector2::zeros(),
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

fn rk4ode_solver(nb: u8, system: &mut State, delta: f64) {
	let mut stage_weight: f64 = 0.0;
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
		theta: Vector2::zeros(),
		v_theta: Vector2::zeros(),
		a_theta: Vector2::zeros(),
		p_x: Vector2::zeros(),
		v_x: Vector2::zeros(),
		a_x: Vector2::zeros(),
		p_y: Vector2::zeros(),
		v_y: Vector2::zeros(),
		a_y: Vector2::zeros(),
		r_x: Vector2::zeros(),
		r_y: Vector2::zeros(),
		r_t: Vector2::zeros(),
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
	mass: f64,
	position: Point2<f64>,
	velocity: Vector2<f64>,
	force_accumulator: Vec<Vector2<f64>>,
	last_velocity: Vector2<f64>,
	last_acceleration: Vector2<f64>,
	radius: f64,
	color: Color,
}
impl Particle {
	pub fn new(mass: f64, position: Point2<f64>, radius: f64, color: Color) -> Self {
		Self {
			mass,
			position,
			velocity: Vector2::zeros(),
			force_accumulator: Vec::new(),
			last_velocity: Vector2::zeros(),
			last_acceleration: Vector2::zeros(),
			radius,
			color,
		}
	}

	pub fn get_position(&self) -> Point2<f64> {
		self.position
	}
	pub fn set_position(&mut self, position: Point2<f64>) {
		self.position = position;
	}

	pub fn get_velocity(&self) -> Vector2<f64> {
		self.velocity
	}
	pub fn set_velocity(&mut self, velocity: Vector2<f64>) {
		self.velocity = velocity;
	}

	pub fn get_mass(&self) -> f64 {
		self.mass
	}

	pub fn apply_force(&mut self, force: Vector2<f64>) {
		self.force_accumulator.push(force);
	}
	pub fn get_force(&self) -> Vector2<f64> {
		self.force_accumulator.clone().into_iter().sum()
	}

	pub fn collide_point(&self, point: Point2<f64>) -> bool {
		(self.position - point).norm() < self.radius
	}

	pub fn update(&mut self, delta: f64) {
		let force = self.get_force();
		let acceleration = force / self.mass;

		// Runge-Kutta 1st order (midpoint method)
		// let average_acceleration = (3.0 * self.last_acceleration - acceleration) / 2.0;
		// self.velocity += average_acceleration * delta;
		// let average_velocity = (self.last_velocity + self.velocity) / 2.0;
		// self.position += average_velocity * delta;

		// Euler
		self.velocity += acceleration * delta;
		self.position += self.velocity * delta;

		// Runge-Kutta 4nd order (RK4)
		/*
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
		*/
		self.last_velocity = self.velocity;
		self.last_acceleration = acceleration;
	}

	pub fn clear_force_accumulator(&mut self) {
		self.force_accumulator.clear();
	}

	pub fn draw(&self, canvas: &mut Canvas<Window>, camera: &Camera) {
		if self.radius == 0.0 {
			return;
		}
		camera.fill_circle(canvas, self.color, self.position, self.radius);
		camera.draw_circle(canvas, Colors::BLACK, self.position, self.radius);
	}

	pub fn draw_forces(&self, canvas: &mut Canvas<Window>, camera: &Camera, scaler: f64) {
		if self.radius == 0.0 {
			return;
		}
		for (i, force) in self.force_accumulator.iter().enumerate() {
			let color = if i == self.force_accumulator.len() - 1 { Colors::VIOLET } else { Colors::LIGHT_YELLOW };
			camera.draw_arrow(canvas, color, self.position, self.position + *force * scaler, 5.0);
		}
	}
}

/// Draw an arrow from start to end with the head at the end.
fn draw_arrow(canvas: &mut Canvas<Window>, color: Color, start: Point2<f64>, end: Point2<f64>, width: f64) {
	if start == end {
		return;
	}
	// TODO clean up
	let x_dir = end - start;
	let y_dir = x_dir.perpendicular() * width / 2.0;
	let linear_transform = Matrix2::new(x_dir.x, y_dir.x, x_dir.y, y_dir.y); // x_dir, y_dir
																		 // let transform = |v: Vector2<f64>| start + linear_transform * v;
	let transform = Transform2::from_matrix_unchecked(Matrix3::new(
		x_dir.x, y_dir.x, start.x, x_dir.y, y_dir.y, start.y, 0.0, 0.0, 1.0,
	));

	let head_back: f64 = 1.0 - 3.0 * width / x_dir.norm();

	let mut points = Vec::from([
		Point2::new(head_back, -1.0),
		Point2::new(head_back, -3.0),
		Point2::new(1.0, 0.0),
		Point2::new(head_back, 3.0),
		Point2::new(head_back, 1.0),
	]);
	if x_dir.norm() > 3.0 * width {
		points.append(&mut Vec::from([Point2::new(0.0, 1.0), Point2::new(0.0, -1.0)]));
	}
	points.iter_mut().for_each(|v| *v = transform * *v);
	let points_x: Vec<i16> = points.iter().map(|v| v.x as i16).collect();
	let points_y: Vec<i16> = points.iter().map(|v| v.y as i16).collect();

	DrawRenderer::filled_polygon(canvas, &points_x, &points_y, color).unwrap();
	DrawRenderer::polygon(canvas, &points_x, &points_y, Colors::BLACK).unwrap();
}
