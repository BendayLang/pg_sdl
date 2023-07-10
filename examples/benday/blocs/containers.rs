use crate::blocs::widgets::TextBox;
use crate::blocs::{Bloc, Skeleton};
use nalgebra::{Point2, Vector2};
use pg_sdl::color::{darker, Colors};
use pg_sdl::prelude::Camera;
use pg_sdl::text::TextDrawer;
use sdl2::pixels::Color;
use sdl2::render::{BlendMode, Canvas};
use sdl2::video::Window;
use std::collections::HashMap;

/// Compartiment d'un bloc.
///
/// Peut contenir du texte où un bloc.
pub struct Slot {
	text_box: TextBox,
	child_id: Option<u32>,
	position: Vector2<f64>,
	size: Vector2<f64>,
}

impl Slot {
	pub const DEFAULT_SIZE: Vector2<f64> = Vector2::new(80.0, 25.0);
	pub const RADIUS: f64 = 2.0;

	pub fn new(color: Color, default_text: String) -> Self {
		Self {
			text_box: TextBox::new(Self::DEFAULT_SIZE, color, default_text),
			child_id: None,
			position: Vector2::zeros(),
			size: Self::DEFAULT_SIZE,
		}
	}

	/*
	pub fn repr(&self, blocs: &HashMap<u32, Box<dyn Bloc>>) -> String {
		if let Some(bloc_id) = self.bloc_id {
			blocs.get(&bloc_id).unwrap().repr(blocs)
		} else {
			let text = self.text_box.get_text();
			if !text.is_empty() {
				text
			} else {
				String::from("-")
			}
		}
	}
	 */

	pub fn get_size(&self) -> Vector2<f64> {
		self.size
	}

	/// Returns a vec of the slot's childs ids from leaf to root (including itself)
	pub fn get_recursive_childs(&self, blocs: &HashMap<u32, Box<dyn Bloc>>) -> Vec<u32> {
		if let Some(bloc_id) = self.child_id {
			blocs.get(&bloc_id).unwrap().get_skeleton().get_recursive_childs(blocs)
		} else {
			Vec::new()
		}
	}

	pub fn update_size(&mut self, blocs: &HashMap<u32, Box<dyn Bloc>>) {
		self.size = if let Some(bloc_id) = self.child_id {
			*blocs.get(&bloc_id).unwrap().get_skeleton().get_size()
		} else {
			self.text_box.get_size()
		};
	}

	pub fn update_child_position(&self, parent_position: Point2<f64>, blocs: &mut HashMap<u32, Box<dyn Bloc>>) {
		if let Some(child_id) = self.child_id {
			blocs.get_mut(&child_id).unwrap().get_skeleton_mut().set_position(parent_position + self.position);
		}
	}

	pub fn set_position(&mut self, position: Vector2<f64>) {
		self.position = position;
	}

	pub fn collide_point(&self, point: Point2<f64>) -> bool {
		self.position.x < point.x
			&& point.x < self.position.x + self.size.x
			&& self.position.y < point.y
			&& point.y < self.position.y + self.size.y
	}

	pub fn collide_rect(&self, position: Point2<f64>, size: Vector2<f64>) -> bool {
		self.position.x < position.x + size.x
			&& position.x < self.position.x + self.size.x
			&& self.position.y < position.y + size.y
			&& position.y < self.position.y + self.size.y
	}

	pub fn get_ratio(&self, position: Point2<f64>, size: Vector2<f64>) -> f64 {
		((position.y + size.y).min(self.position.y + self.size.y) - position.y.max(self.position.y)) / self.size.y
	}

	pub fn has_child(&self) -> bool {
		self.child_id.is_some()
	}

	/// Renvoie la référence du bloc en collision avec un point et sur quelle partie du bloc est ce point.
	/*
	pub fn collide_point(
		&self, point: Vector2<f64>, slot_id: usize, blocs: &HashMap<u32, Box<dyn Bloc>>,
	) -> Option<(Vec<usize>, HoveredOn)> {
		if let Some(bloc_id) = self.bloc_id {
			if let Some(bloc_collide) = blocs.get(&bloc_id).unwrap().collide_point(point, blocs) {
				let (mut hierarchy, hovered_on) = bloc_collide;
				hierarchy.push(slot_id);
				return Some((hierarchy, hovered_on));
			} else {
				return None;
			}
		} else {
			if self.collide(point) {
				return Some((Vec::new(), HoveredOn::Slot(slot_id)));
			}
			return None;
		}
	}
	*/

	/// Renvoie la référence du slot en collision avec un rectangle et la proportion de collision.
	/*
	pub fn hovered_slot(
		&self, position: Point2<f64>, size: Vector2<f64>, ratio: f32, slot_id: usize, blocs: &HashMap<u32, Box<dyn Bloc>>,
	) -> Option<(Vec<usize>, f32)> {
		if !self.colliderect(position, size) {
			return None;
		}

		if let Some(bloc_id) = self.bloc_id {
			if let Some((mut hierarchy, new_ratio)) =
				blocs.get(&bloc_id).unwrap().hovered_slot(position, size, ratio, blocs)
			{
				if new_ratio > ratio {
					hierarchy.push(slot_id);
					return Some((hierarchy, new_ratio));
				}
			}
		} else {
			let height_collision = position.y.max(0.0) - (position.y + size.y).min(self.size.y);
			let new_ratio = (height_collision / self.size.y) as f32;
			if new_ratio > ratio {
				return Some((vec![slot_id], new_ratio));
			}
		}
		return None;
	}
	 */

	/// Vide le slot de son contenu.
	pub fn set_empty(&mut self) {
		self.child_id = None;
		/*
		self.text_box.size.y = Self::DEFAULT_SIZE.y;
		self.text_box.update_size(camera);
		self.text_box.corner_radius = Self::RADIUS;
		self.text_box.hovered = false;
		 */
	}

	/// Définit le slot comme étant survolé.
	fn set_hovered(&mut self, size: Vector2<f64>) {
		/*
		self.text_box.size = size.copy();
		self.text_box.corner_radius = Bloc::RADIUS;
		self.text_box.hovered = true;
		 */
	}

	/// Ajoute un bloc enfant donné dans le slot.
	pub fn set_child(&mut self, child_id: u32) {
		self.child_id = Some(child_id);
	}

	/// Affiche le slot.
	pub fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &TextDrawer, camera: &Camera, position: Point2<f64>) {
		if self.child_id.is_none() {
			self.text_box.draw(canvas, text_drawer, camera, position + self.position);
		}
	}

	pub fn draw_selected(&self, canvas: &mut Canvas<Window>, camera: &Camera, position: Point2<f64>) {
		if self.child_id.is_none() {
			camera.draw_rounded_rect(canvas, Colors::BLACK, position + self.position, self.size, Self::RADIUS);
		}
	}

	pub fn draw_hovered(&self, canvas: &mut Canvas<Window>, camera: &Camera, position: Point2<f64>) {
		if self.child_id.is_none() {
			canvas.set_blend_mode(BlendMode::Mod);
			camera.fill_rounded_rect(
				canvas,
				Color::from((0, 0, 0, Skeleton::HOVER_ALPHA)),
				position + self.position,
				self.size,
				Self::RADIUS,
			);
			canvas.set_blend_mode(BlendMode::None);
		}
	}

	// Retourne l’ASTNode de la séquence.
	/*
	fn as_ast(&self, blocs: &HashMap<u32, Bloc>) -> ASTNodeValue {
		if let Some(bloc_id) = self.bloc_id {
			bloc_id
		} else {
			ASTNodeValue(if self.text_box.text.is_empty() { None } else { Some(&self.text_box.text) })
		}
	}
	 */
}

pub struct Sequence {
	color: Color,
	blocs_ids: Vec<u32>,
	position: Vector2<f64>,
	size: Vector2<f64>,
}

impl Sequence {
	const DEFAULT_SIZE: Vector2<f64> = Vector2::new(120.0, 80.0);
	const MARGIN: f64 = 7.0;
	const RADIUS: f64 = 10.0;

	/*
	pub fn repr(self, blocs: &HashMap<u32, Box<dyn Bloc>>) -> String {
		self.blocs_ids.iter().map(|bloc_id| blocs.get(bloc_id).unwrap().repr(blocs)).collect::<Vec<_>>().join(" , ")
	}
	 */

	pub fn get_position(&self) -> Vector2<f64> {
		self.position
	}

	/// Retourne la taille de la séquence.
	fn get_size(self, blocs: &HashMap<u32, Box<dyn Bloc>>) -> Vector2<f64> {
		if self.blocs_ids.is_empty() {
			Self::DEFAULT_SIZE
		} else {
			let nb_blocs = self.blocs_ids.len();
			let width = self
				.blocs_ids
				.iter()
				.map(|bloc_id| blocs.get(bloc_id).unwrap().get_skeleton().get_size().x)
				.max_by(|a, b| a.partial_cmp(b).unwrap())
				.unwrap();
			let height = (self
				.blocs_ids
				.iter()
				.map(|bloc_id| blocs.get(bloc_id).unwrap().get_skeleton().get_size().y)
				.sum::<f64>())
			.max(Self::DEFAULT_SIZE.y);
			// let width = max([self.bloc_size(i).x for i in 0..nb_blocs]);
			// let height = max(sum([self.bloc_size(i).y for i in 0..nb_blocs]), Self::DEFAULT_SIZE.y);
			Vector2::new(width, height) + Vector2::new(1, nb_blocs).cast() * Self::MARGIN
		}
	}

	/// Renvoie la position du bloc donné en référence à la séquence parent.
	fn bloc_position(&self, nth_bloc: usize, blocs: &HashMap<u32, Box<dyn Bloc>>) -> Vector2<f64> {
		Vector2::new(
			0.0,
			(0..nth_bloc)
				.map(|i| blocs.get(self.blocs_ids.get(i).unwrap()).unwrap().get_skeleton().size.y + Self::MARGIN)
				.sum(),
		)
	}

	/// Met à jour la taille de la séquence.
	pub fn update_size(&self, blocs: &mut HashMap<u32, Box<dyn Bloc>>) {
		// self.blocs_ids.iter().for_each(|bloc_id| blocs.get(bloc_id).unwrap().update_size(blocs));
	}

	pub fn collide_point(&self, point: Point2<f64>) -> bool {
		self.position.x < point.x
			&& point.x < self.position.x + self.size.x
			&& self.position.y < point.y
			&& point.y < self.position.y + self.size.y
	}

	pub fn collide_rect(&self, position: Point2<f64>, size: Vector2<f64>) -> bool {
		self.position.x < position.x + size.x
			&& position.x < self.position.x + self.size.x
			&& self.position.y < position.y + size.y
			&& position.y < self.position.y + self.size.y
	}

	/// Renvoie la référence du bloc en collision avec un point et sur quelle partie du bloc est ce point.
	/*
	pub fn collide_point(
		&self, point: Vector2<f64>, sequence_id: usize, blocs: &HashMap<u32, Box<dyn Bloc>>,
	) -> Option<(Vec<usize>, HoveredOn)> {
		for (i, bloc_id) in self.blocs_ids.iter().rev().enumerate() {
			let nth_bloc = self.blocs_ids.len() - 1 - i;
			let bloc_position = self.bloc_position(nth_bloc, blocs);

			if let Some(bloc_collide) = blocs.get(bloc_id).unwrap().collide_point(point - bloc_position, blocs) {
				let (mut hierarchy, hovered_on) = bloc_collide;
				hierarchy.push(sequence_id); // (sequence_id, nth_bloc)
				return Some((hierarchy, hovered_on));
			}
		}

		if self.collide(point) {
			return Some((Vec::new(), HoveredOn::Sequence(sequence_id)));
		}
		None
	}
	 */

	/// Renvoie la référence du slot en collision avec un rectangle et la proportion de collision.
	/*
	pub fn hovered_slot(
		&self, position: Point2<f64>, size: Vector2<f64>, ratio: f32, sequence_id: usize,
		blocs: &HashMap<u32, Box<dyn Bloc>>,
	) -> Option<(Vec<usize>, f32)> {
		if !self.colliderect(position, size) {
			return None;
		}
		let mut hierarchy_ratio = None;

		if self.blocs_ids.is_empty() {
			let delta = position.y - self.bloc_position(0, blocs).y;
			let new_ratio = (1.0 - (delta / size.y).abs()).max(0.1) as f32;
			if new_ratio > ratio {
				let ratio = new_ratio;
				hierarchy_ratio = Some((vec![sequence_id], new_ratio)); // (sequence_id, 0)
			}
		}

		if let Some(hovered_where) = self.is_hovered_where() {
			(0..=self.blocs_ids.len()).for_each(|i| {
				if i as usize != hovered_where + 1 {
					let delta = position.y - self.bloc_position(i, blocs).y;

					let gap_size = if i + 1 == self.blocs_ids.len() {
						self.size.y + Self::MARGIN - self.bloc_position(i, blocs).y
					} else if i as usize == hovered_where {
						self.bloc_position(i + 1, blocs).y - self.bloc_position(i, blocs).y
					} else {
						Self::MARGIN
					};

					if delta + size.y >= 0.0 && delta <= gap_size {
						let new_ratio = (1.0 - (delta / size.y).abs()).max(0.1) as f32;
						if new_ratio > ratio {
							let ratio = new_ratio;
							let mut bloc_id = i;
							if i as usize > hovered_where {
								bloc_id -= 1;
							}
							hierarchy_ratio = Some((vec![sequence_id], new_ratio)) // (sequence_id, bloc_id)
						}
					}
				}
			});
			self.blocs_ids.iter().enumerate().for_each(|(i, bloc_id)| {
				if i as usize != hovered_where {
					if let Some((mut hierarchy, new_ratio)) = blocs.get(bloc_id).unwrap().hovered_slot(
						position - self.bloc_position(i, blocs),
						size,
						ratio,
						blocs,
					) {
						if new_ratio > ratio {
							let ratio = new_ratio;
							let mut nth_bloc = i;
							if i as usize > hovered_where {
								nth_bloc -= 1
							}
							hierarchy.push(sequence_id); // (sequence_id, nth_bloc)
							hierarchy_ratio = Some((hierarchy, new_ratio));
						}
					}
				}
			});
		};
		hierarchy_ratio
	}
	 */

	/// Retourne l’id du gap survolé par un point donné (pour savoir où ajouter un nouveau bloc).
	fn hovered_gap(self, point: Point2<f64>, blocs: &HashMap<u32, Box<dyn Bloc>>) -> usize {
		if self.blocs_ids.is_empty() {
			return 0;
		} else {
			for nth_bloc in 0..self.blocs_ids.len() {
				if point.y
					< self.bloc_position(nth_bloc, blocs).y
						+ blocs.get(&self.blocs_ids[nth_bloc]).unwrap().get_skeleton().size.y * 0.5
				{
					return nth_bloc;
				}
			}
			return self.blocs_ids.len();
		}
	}

	/// Renvoie l’id de l’espace au-dessus duquel le bloc est survolé.
	fn is_hovered_where(&self) -> Option<usize> {
		return None;
	}

	/// Enlève le bloc donné de la séquence.
	fn set_empty(&mut self, nth_bloc: usize) {
		self.blocs_ids.remove(nth_bloc);
	}

	/// Ajoute un espace à une position donnée.
	fn set_hovered(&mut self, gap_id: usize, bloc_id: u32) {
		// self.blocs_ids.insert(gap_id, bloc_id);
	}

	/// Ajoute un bloc donné à une position donnée dans la séquence.
	fn set_child(&mut self, gap_id: usize, bloc_id: u32) {
		/*
		if gap_id == self.blocs_ids.len() {
			self.blocs_ids.last().unwrap() = bloc_id;
		} else {
			self.blocs_ids[gap_id] = bloc_id;
		}
		 */
	}

	/// Affiche la séquence.
	pub fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &TextDrawer, camera: &Camera, position: Point2<f64>) {
		camera.fill_rounded_rect(canvas, darker(self.color, 0.7), position + self.position, self.size, Self::RADIUS);
	}
}

/*
	fn as_AST(self) -> ASTNodeSequence:
		"""Retourne la list contenant les ASTNodes de la séquence."""
		return ASTNodeSequence([bloc.as_ASTNode() for bloc in self.blocs])
*/

/*
	fn bloc_size(self, bloc_id: int) -> Vector2<f64>:
		"""Retourne la taille du bloc donné."""
		return self.blocs[bloc_id] if type(self.blocs[bloc_id]) is Vec2 else self.blocs[bloc_id].size
*/
