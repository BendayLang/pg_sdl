pub mod containers;
pub mod print;
mod variable_assignment;
mod widgets;

use crate::blocs::containers::{Sequence, Slot};
use crate::Container;
use nalgebra::{Point2, Vector2};
use pg_sdl::camera::Camera;
use pg_sdl::color::Colors;
use pg_sdl::prelude::{Align, TextDrawer};
use sdl2::pixels::Color;
use sdl2::render::{BlendMode, Canvas};
use sdl2::video::Window;
use std::collections::HashMap;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum BlocElement {
	Body,
	DeleteButton,
	CopyButton,
	InfoButton,
	Slot(usize),
	Sequence(usize),
	CustomButton(usize),
}

#[derive(PartialEq, Debug, Clone)]
pub enum BlocContainer {
	Slot { slot_id: usize },
	Sequence { sequence_id: usize, place: usize },
}

pub trait Bloc {
	// fn new(position: Point2<f64>, blocs: &HashMap<u32, Box<dyn Bloc>>) -> Self;
	fn get_skeleton(&self) -> &Skeleton;
	fn get_skeleton_mut(&mut self) -> &mut Skeleton;
	// fn reset_size_and_position(&mut self, blocs: &HashMap<u32, Box<dyn Bloc>>);

	// fn get_size(&self, blocs: &HashMap<u32, Box<dyn Bloc>>) -> Vector2<f64>;
	// fn slot_position(&self, slot_id: usize) -> Vector2<f64>;
	// fn sequence_position(&self, sequence_id: usize) -> Vector2<f64>;
	fn button_size(&self, button_id: usize) -> Vector2<f64>;
	fn button_position(&self, button_id: usize) -> Vector2<f64>;
	fn draw_button(&self, canvas: &mut Canvas<Window>, text_drawer: &TextDrawer, camera: &Camera);
	fn button_function(&mut self, button_id: usize) -> bool;
}

/// A bloc represents a piece of code that can be executed.
///
/// It has a "skeleton" that contains everything that all blocs have in common:
/// - color
/// - position (it's always absolute)
/// - vector of slots
/// - vector of sequences
///
/// And a bloc type, witch is an enum that contains data specific to the bloc.
pub struct Skeleton {
	id: u32,
	color: Color,
	position: Point2<f64>,
	size: Vector2<f64>,
	slots: Vec<Slot>,
	slots_positions: Box<dyn Fn(usize) -> Vector2<f64>>,
	sequences: Vec<Sequence>,
	sequences_positions: Box<dyn Fn(usize) -> Vector2<f64>>,
	get_size: Box<dyn Fn(&Skeleton) -> Vector2<f64>>,
	parent: Option<Container>,
}
impl Skeleton {
	pub const RADIUS: f64 = 8.0;
	const MARGIN: f64 = 12.0;
	const INNER_MARGIN: f64 = 6.0;
	const SHADOW: Vector2<f64> = Vector2::new(6.0, 8.0);
	const HOVER_ALPHA: u8 = 20;

	pub fn new(
		id: u32, color: Color, position: Point2<f64>, slots: Vec<Slot>,
		slots_positions: Box<dyn Fn(usize) -> Vector2<f64>>, sequences: Vec<Sequence>,
		sequences_positions: Box<dyn Fn(usize) -> Vector2<f64>>, get_size: Box<dyn Fn(&Skeleton) -> Vector2<f64>>,
	) -> Self {
		let mut skeleton = Self {
			id,
			color,
			position,
			size: Vector2::zeros(),
			slots,
			slots_positions,
			sequences,
			sequences_positions,
			get_size,
			parent: None,
		};
		(0..skeleton.slots.len())
			.for_each(|slot_id| skeleton.slots[slot_id].set_position((*skeleton.slots_positions)(slot_id)));
		skeleton.size = (*skeleton.get_size)(&skeleton);
		skeleton
	}

	/*
	pub fn repr(&self, blocs: &HashMap<u32, Box<dyn Bloc>>) -> String {
		format!("Bloc( {} )", self.slots.get(0).unwrap().repr(blocs))
	}
	 */

	pub fn set_parent(&mut self, parent: Option<Container>) {
		self.parent = parent
	}

	pub fn get_parent(&self) -> &Option<Container> {
		&self.parent
	}

	pub fn set_position(&mut self, position: Point2<f64>) {
		self.position = position
	}

	pub fn get_position(&self) -> &Point2<f64> {
		&self.position
	}

	pub fn get_size(&self) -> &Vector2<f64> {
		&self.size
	}

	/// Returns a vec of the bloc's childs ids from leaf to root (including itself)
	pub fn get_recursive_childs(&self, blocs: &HashMap<u32, Box<dyn Bloc>>) -> Vec<u32> {
		let mut childs = Vec::new();
		self.slots.iter().for_each(|slot| {
			childs.extend(slot.get_recursive_childs(blocs));
		});
		childs.push(self.id);
		childs
	}

	/// Met à jour la taille du bloc et la position de ses slots et séquences
	pub fn update_layout(&mut self, blocs: &HashMap<u32, Box<dyn Bloc>>) {
		self.slots.iter_mut().for_each(|slot| slot.update_size(blocs));
		(0..self.slots.len()).for_each(|slot_id| self.slots[slot_id].set_position((*self.slots_positions)(slot_id)));
		// TODO same for sequences
		self.size = (*self.get_size)(&self);
	}

	/// Met à jour la position de ses enfants
	pub fn update_child_position(&self, blocs: &mut HashMap<u32, Box<dyn Bloc>>) {
		self.slots.iter().for_each(|slot| slot.update_child_position(self.position, blocs));
		// TODO same for sequences
	}

	pub fn collide_element(&self, point: Point2<f64>) -> Option<BlocElement> {
		if !self.collide_point(point) {
			return None;
		}

		for (slot_id, slot) in self.slots.iter().enumerate() {
			if slot.collide_point(point - self.position.coords) {
				return Some(BlocElement::Slot(slot_id));
			}
		}

		for (sequence_id, sequence) in self.sequences.iter().enumerate() {
			if sequence.collide_point(point - self.position.coords) {
				return Some(BlocElement::Sequence(sequence_id));
			}
		}

		Some(BlocElement::Body)
	}

	pub fn collide_container(&self, position: Point2<f64>, size: Vector2<f64>) -> Option<(BlocContainer, f64)> {
		if !self.collide_rect(position, size) {
			return None;
		}

		let (mut bloc_container, mut ratio) = (None, 0.0);

		self.slots.iter().enumerate().for_each(|(slot_id, slot)| {
			if slot.collide_rect(position - self.position.coords, size) && !slot.has_child() {
				let new_ratio = slot.get_ratio(position - self.position.coords, size);
				if new_ratio > ratio {
					bloc_container = Some(BlocContainer::Slot { slot_id });
					ratio = new_ratio;
				}
			}
		});

		// TODO idem for sequences

		if let Some(bloc_container) = bloc_container {
			return Some((bloc_container, ratio));
		}
		None
	}

	pub fn set_slot_child(&mut self, slot_id: usize, child_id: u32) {
		self.slots[slot_id].set_child(child_id);
	}

	pub fn set_slot_empty(&mut self, slot_id: usize) {
		self.slots[slot_id].set_empty();
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

	/// Retourne la référence du bloc en collision avec un point (hiérarchie)
	///
	/// et sur quelle partie du bloc est ce point (hovered on).
	/*
	fn collide_point(&self, point: Vector2<f64>, blocs: &HashMap<u32, Box<dyn Bloc>>) -> Option<(Vec<usize>, HoveredOn)> {
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
			let slot_id = (self.slots.len() - 1 - i) as usize;
			let slot_collide = slot.collide_point(point - self.slot_position(slot_id), slot_id, blocs);
			if slot_collide.is_some() {
				return slot_collide;
			}
		}

		for (i, sequence) in self.sequences.iter().rev().enumerate() {
			let sequence_id = (self.sequences.len() - 1 - i) as usize;
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
	 */

	/// Retourne la référence du slot ou de la séquence en collision avec un rectangle (hiérarchie)
	///
	/// et la proportion de collision en hauteur (float).
	/*
	fn hovered_slot(
		&self, position: Point2<f64>, size: Vector2<f64>, ratio: f32, blocs: &HashMap<u32, Box<dyn Bloc>>,
	) -> Option<(Vec<usize>, f32)> {
		if !(Self::MARGIN - size.x < position.x
			&& position.x < self.size.x - 2.0 * Self::MARGIN
			&& Self::MARGIN - size.y < position.y
			&& position.y < self.size.y - 2.0 * Self::MARGIN)
		{
			return None;
		}
		let mut hierarchy_ratio = None;

		self.slots.iter().enumerate().for_each(|(slot_id, slot)| {
			let slot_position = self.slot_position(slot_id as usize);

			if let Some(slot_hovered) = slot.hovered_slot(position - slot_position, size, ratio, slot_id as usize, blocs)
			{
				let (hierarchy, ratio) = slot_hovered;
				hierarchy_ratio = Some((hierarchy, ratio));
			}
		});

		self.sequences.iter().enumerate().for_each(|(sequence_id, sequence)| {
			let sequence_position = self.sequence_position(sequence_id as usize);

			if let Some(sequence_hovered) =
				sequence.hovered_slot(position - sequence_position, size, ratio, sequence_id as usize, blocs)
			{
				let (hierarchy, ratio) = sequence_hovered;
				hierarchy_ratio = Some((hierarchy, ratio));
			}
		});

		hierarchy_ratio
	}
	 */

	pub fn draw(
		&self, canvas: &mut Canvas<Window>, text_drawer: &TextDrawer, camera: &Camera, moving: bool,
		selected: Option<&BlocElement>, hovered: Option<&BlocElement>,
	) {
		// SHADOW
		let origin = if moving {
			let shadow_color = Color::from((0, 0, 0, 50));
			canvas.set_blend_mode(BlendMode::Mod);
			camera.fill_rounded_rect(canvas, shadow_color, self.position, self.size, Self::RADIUS);
			canvas.set_blend_mode(BlendMode::None);
			self.position - Self::SHADOW
		} else {
			self.position
		};
		// BODY
		camera.fill_rounded_rect(canvas, self.color, origin, self.size, Self::RADIUS);
		if selected.is_some() || hovered.is_some() {
			// draw top box
		}
		// SLOTS
		self.slots.iter().for_each(|slot| {
			slot.draw(canvas, text_drawer, camera, origin);
		});
		// SEQUENCES
		self.sequences.iter().for_each(|sequence| {
			sequence.draw(canvas, text_drawer, camera, origin);
		});
		// HOVERED
		if let Some(element) = hovered {
			match element {
				BlocElement::Body => {
					canvas.set_blend_mode(BlendMode::Mod);
					camera.fill_rounded_rect(
						canvas,
						Color::from((0, 0, 0, Self::HOVER_ALPHA)),
						origin,
						self.size,
						Self::RADIUS,
					);
					canvas.set_blend_mode(BlendMode::None);
				}
				BlocElement::Slot(slot_id) => {
					let slot = self.slots.get(slot_id.clone()).unwrap();
					slot.draw_hovered(canvas, camera, origin);
				}
				_ => (),
			}
		}
		// SELECTED
		if let Some(element) = selected {
			match element {
				BlocElement::Body => {
					camera.draw_rounded_rect(canvas, Colors::BLACK, origin, self.size, Self::RADIUS);
				}
				BlocElement::Slot(slot_id) => {
					let slot = self.slots.get(slot_id.clone()).unwrap();
					slot.draw_selected(canvas, camera, origin);
				}
				_ => (),
			}
		}
		let text = format!("{}  parent: {:?}", self.id, self.parent);
		camera.draw_text(canvas, text_drawer, origin, 15.0, text, Align::TopLeft);
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
