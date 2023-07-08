mod containers;
mod variable_assignment;
mod widgets;

use crate::blocs::containers::{HoveredOn, Sequence, Slot};
use crate::blocs::widgets::TextBox;
use as_any::AsAny;
use nalgebra::{Point2, Vector2};
use pg_sdl::camera::Camera;
use pg_sdl::color::{paler, Colors};
use pg_sdl::prelude::TextDrawer;
use sdl2::pixels::Color;
use sdl2::render::{BlendMode, Canvas};
use sdl2::video::Window;
use std::collections::HashMap;

pub trait Bloc {
	fn get_size(&self, blocs: &HashMap<u32, Box<dyn Bloc>>) -> Vector2<f64>;
}

pub struct Print {
	skeleton: Skeleton,
}

impl Bloc for Print {
	fn get_size(&self, blocs: &HashMap<u32, Box<dyn Bloc>>) -> Vector2<f64> {
		self.skeleton.slots.get(0).unwrap().get_size(blocs) + Vector2::new(2.0, 2.0) // * Self::MARGIN
	}
}

/// A bloc represents a piece of code that can be executed.
///
/// It has a "skeleton" that contains everything that all blocs have in common:
/// - color
/// - position (if the bloc has a parent, it's relative to the parent)
/// - vector of slots
/// - vector of sequences
///
/// And a bloc type, witch is an enum that contains data specific to the bloc.
pub struct Skeleton {
	color: Color,
	pub position: Point2<f64>,
	pub size: Vector2<f64>,
	hovered_on: HoveredOn,
	slots: Vec<Slot>,
	sequences: Vec<Sequence>,
}
impl Skeleton {
	const RADIUS: f64 = 8.0;
	const MARGIN: f64 = 12.0;
	const SHADOW: Vector2<f64> = Vector2::new(6.0, 8.0);

	pub fn new_variable_assignment(color: Color, position: Point2<f64>, blocs: &HashMap<u32, Skeleton>) -> Self {
		// let color = hsv_color(30, 0.6, 1.0);
		let mut bloc = Self {
			color,
			position,
			size: Vector2::zeros(),
			hovered_on: HoveredOn::None,
			slots: vec![Slot::new(color, "value")],
			sequences: Vec::new(),
			bloc_type: BlocType::VariableAssignment {
				name: TextBox::new(Slot::DEFAULT_SIZE, paler(color, 0.1), "name".to_string()),
			},
		};
		bloc.size = bloc.get_size(blocs);
		bloc
	}

	pub fn repr(&self, blocs: &HashMap<u32, Skeleton>) -> String {
		format!("Bloc( {} )", self.slots.get(0).unwrap().repr(blocs))
	}

	/// Met à jour la taille du bloc et celles de ses enfants.
	fn update_size(&mut self, blocs: &mut HashMap<u32, Skeleton>) {
		/*
		self.slots.iter().for_each(|slot| slot.update_size(blocs));
		self.sequences.iter().for_each(|sequence| sequence.update_size(blocs));
		self.size = self.get_size();
		 */
	}
	/// Retourne la taille du bloc.
	fn get_size(&self, blocs: &HashMap<u32, Skeleton>) -> Vector2<f64> {
		// panic!("'get_size' is not implemented in '{}' class", self.type_name())
		self.slots.get(0).unwrap().get_size(blocs) + Vector2::new(2.0, 2.0) * Self::MARGIN
	}

	pub fn collide(&self, point: Vector2<f64>) -> bool {
		self.position.x < point.x
			&& point.x < self.position.x + self.size.x
			&& self.position.y < point.y
			&& point.y < self.position.y + self.size.y
	}

	pub fn collide_bloc(&self, bloc: &Skeleton) -> bool {
		self.position.x < bloc.position.x + bloc.size.x
			&& bloc.position.x < self.position.x + self.size.x
			&& self.position.y < bloc.position.y + bloc.size.y
			&& bloc.position.y < self.position.y + self.size.y
	}

	/// Retourne la référence du bloc en collision avec un point (hiérarchie)
	///
	/// et sur quelle partie du bloc est ce point (hovered on).
	fn collide_point(&self, point: Vector2<f64>, blocs: &HashMap<u32, Skeleton>) -> Option<(Vec<u16>, HoveredOn)> {
		if !self.collide(point) {
			return None;
		}
		/*
		for button_id in 0..self.buttons.len(){
			if self.collide_button(point, button_id) {
				return Some((Vec::new(), HoveredOn::CustomButton(button_id)))
			}
		}
		*/
		for (i, slot) in self.slots.iter().rev().enumerate() {
			let slot_id = (self.slots.len() - 1 - i) as u16;
			let slot_collide = slot.collide_point(point - self.slot_position(slot_id), slot_id, blocs);
			if slot_collide.is_some() {
				return slot_collide;
			}
		}

		for (i, sequence) in self.sequences.iter().rev().enumerate() {
			let sequence_id = (self.sequences.len() - 1 - i) as u16;
			let sequence_collide =
				sequence.collide_point(point - self.sequence_position(sequence_id), sequence_id, blocs);
			if sequence_collide.is_some() {
				return sequence_collide;
			}
		}
		/*
		if self.collide_info_bt(point){
			return Some((Vec::new(), HoveredOn::InfoButton));
		}

		if self.collide_copy_bt(point){
			return Some((Vec::new(), HoveredOn::CopyButton));
		}

		if self.collide_cross_bt(point){
			return Some((Vec::new(), HoveredOn::DeleteButton));
		}
		*/
		return Some((Vec::new(), HoveredOn::Body));
	}

	fn slot_position(&self, slot_id: u16) -> Vector2<f64> {
		Vector2::new(Self::MARGIN, Self::MARGIN)
	}

	fn sequence_position(&self, sequence_id: u16) -> Vector2<f64> {
		Vector2::zeros()
	}

	/// Retourne la référence du slot ou de la séquence en collision avec un rectangle (hiérarchie)
	///
	/// et la proportion de collision en hauteur (float).
	fn hovered_slot(
		&self, position: Point2<f64>, size: Vector2<f64>, ratio: f32, blocs: &HashMap<u32, Skeleton>,
	) -> Option<(Vec<u16>, f32)> {
		if !(Self::MARGIN - size.x < position.x
			&& position.x < self.size.x - 2.0 * Self::MARGIN
			&& Self::MARGIN - size.y < position.y
			&& position.y < self.size.y - 2.0 * Self::MARGIN)
		{
			return None;
		}
		let mut hierarchy_ratio = None;

		self.slots.iter().enumerate().for_each(|(slot_id, slot)| {
			let slot_position = self.slot_position(slot_id as u16);

			if let Some(slot_hovered) = slot.hovered_slot(position - slot_position, size, ratio, slot_id as u16, blocs)
			{
				let (hierarchy, ratio) = slot_hovered;
				hierarchy_ratio = Some((hierarchy, ratio));
			}
		});

		self.sequences.iter().enumerate().for_each(|(sequence_id, sequence)| {
			let sequence_position = self.sequence_position(sequence_id as u16);

			if let Some(sequence_hovered) =
				sequence.hovered_slot(position - sequence_position, size, ratio, sequence_id as u16, blocs)
			{
				let (hierarchy, ratio) = sequence_hovered;
				hierarchy_ratio = Some((hierarchy, ratio));
			}
		});

		hierarchy_ratio
	}

	pub fn draw(
		&self, canvas: &mut Canvas<Window>, text_drawer: &TextDrawer, camera: &Camera, blocs: &HashMap<u32, Skeleton>,
		selected: bool,
	) {
		// SHADOW
		let origin = if selected {
			let shadow_color = Color::from((0, 0, 0, 50));
			canvas.set_blend_mode(BlendMode::Mod);
			camera.fill_rounded_rect(canvas, shadow_color, self.position, self.size, Self::RADIUS);
			canvas.set_blend_mode(BlendMode::None);
			self.position - Self::SHADOW
		} else {
			self.position
		};

		camera.fill_rounded_rect(canvas, self.color, origin, self.size, Self::RADIUS);
		if self.hovered_on != HoveredOn::None {
			camera.draw_rounded_rect(canvas, Colors::BLACK, origin, self.size, Self::RADIUS);
			// draw top box
		}

		self.slots.iter().enumerate().for_each(|(slot_id, slot)| {
			let hovered = if let HoveredOn::Slot(hovered_slot_id) = self.hovered_on {
				slot_id as u16 == hovered_slot_id
			} else {
				false
			};
			slot.draw(canvas, text_drawer, camera, origin.coords + self.slot_position(slot_id as u16), hovered, blocs);
		});

		self.sequences.iter().enumerate().for_each(|(sequence_id, sequence)| {
			let hovered = if let HoveredOn::Sequence(hovered_sequence_id) = self.hovered_on {
				sequence_id as u16 == hovered_sequence_id
			} else {
				false
			};
			sequence.draw(
				canvas,
				text_drawer,
				camera,
				origin.coords + self.sequence_position(sequence_id as u16),
				hovered,
				blocs,
			);
		});
	}
}

/*
pub fn draw_bloc<'a>(
	bloc: &Bloc, blocs: &HashMap<u32, Bloc>, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer,
	texture_creator: &'a TextureCreator<WindowContext>,
) -> Surface<'a> {
	let mut surface =
		Surface::new(bloc.rect.width(), bloc.rect.height(), texture_creator.default_pixel_format()).unwrap();
	surface.fill_rect(rect!(0, 0, bloc.rect.width(), bloc.rect.height()), bloc.color).unwrap();

	if let Some(child_id) = bloc.child {
		let bloc = blocs.get(&child_id).unwrap();
		let child_surface = draw_bloc(bloc, blocs, canvas, text_drawer, texture_creator);
		child_surface.blit(None, surface.as_mut(), Some(bloc.rect)).unwrap();
	}

	surface
}
 */

/*
pub fn set_child(child_id: u32, parent_id: u32, blocs: &mut HashMap<u32, Bloc>) {
	let child_size = blocs.get(&child_id).unwrap().size;
	{
		let parent_bloc = blocs.get_mut(&parent_id).unwrap();
		parent_bloc.child = Some(child_id);

		parent_bloc.size = parent_bloc.size + child_size;
	}

	let parent_size = blocs.get(&parent_id).unwrap().size;
	{
		let child_bloc = blocs.get_mut(&child_id).unwrap();
		child_bloc.parent = Some(parent_id);

		child_bloc.position = Point2::from((parent_size - child_size) * 0.5);
	}
}
*/
