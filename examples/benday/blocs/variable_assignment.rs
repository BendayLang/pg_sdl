use crate::blocs::widgets::TextBox;
use crate::blocs::{Bloc, Skeleton};
use pg_sdl::color::{Colors, hsv_color, paler};
use std::collections::HashMap;
use nalgebra::{Point2, Vector2};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use pg_sdl::camera::Camera;
use pg_sdl::prelude::TextDrawer;
use crate::blocs::Slot;
// Login admin labo
// name:     .\adminlabo
// password: Robotique2023

/// Bloc d’assignation de variable - la variable nommée sur le côté gauche du bloc
/// prend la valeur de ce que contient le slot du côté droit.
pub struct VariableAssignment {
	skeleton: Skeleton,
	name: TextBox,
	// type: str | None
}

impl VariableAssignment {
	const COLOR: Color = hsv_color(30, 0.6, 1.0);
	
	pub fn new(position: Point2<f64>, blocs: &HashMap<u32, Box<dyn Bloc>>) -> Self {
		let mut bloc = Self {
			skeleton: Skeleton::new(Self::COLOR, position, vec![Slot::new(Self::COLOR, "value")], Vec::new()),
			name: TextBox::new(Slot::DEFAULT_SIZE, Self::COLOR, "name".to_string()),
		};
		bloc.skeleton.size = bloc.get_size(blocs);
		bloc
	}
	/*
	pub fn repr(&self, blocs: &HashMap<u32, Box<dyn Bloc>>) -> String {
		let name = if self.name.get_text().is_empty() { "-".to_string() } else { self.name.get_text() };
		format!("VariableAssignment({}: ? = {})", name, self.bloc.slots[0].repr(blocs))
	}
	 */
}

impl Bloc for VariableAssignment {
	fn get_size(&self, blocs: &HashMap<u32, Box<dyn Bloc>>) -> Vector2<f64> {
		let width = self.skeleton.slots.get(0).unwrap().get_size(blocs).x + self.name.get_size().x + Skeleton::INNER_MARGIN;
	   let height = sum([slot.size.y for slot in self.slots]) + max(len(self.slots) - 1, 0) * Skeleton::INNER_MARGIN;
	   return Vec2(width, height) + Vec2(2 * MARGIN)
	}
	
	fn slot_position(&self, slot_id: u16) -> Vector2<f64> {
		position_x = self.name_box.size.x + self.text_width + TEXT_EQUAL_SIZE.x + 3 * INNER_MARGIN
	   position_y = sum([slot.size.y for slot in self.slots[:slot_id]]) + slot_id * INNER_MARGIN
	   return Vec2(position_x, position_y) + Vec2(MARGIN)
	}
	
	fn sequence_position(&self, sequence_id: u16) -> Vector2<f64> {
		todo!()
	}
	
	fn button_size(&self, button_id: u16) -> Vector2<f64> {
		match self.buttons[button_id]:
		   case "name_box":
			   return self.name_box.size
		   case "choose_type":
			   return Vec2(self.text_width, BT_TYPE_SIZE.y)
	}
	
	fn button_position(&self, button_id: u16) -> Vector2<f64> {
		match self.buttons[button_id]:
		   case "name_box":
			   return Vec2(MARGIN, (self.size.y - self.name_box.size.y) / 2)
		   case "choose_type":
			   return Vec2(MARGIN + self.name_box.size.x + INNER_MARGIN,
						   (self.size.y - BT_TYPE_SIZE.y) / 2)
	}
	
	fn draw_button(&self, canvas: &mut Canvas<Window>, text_drawer: &TextDrawer, camera: &Camera) {
		todo!()
	}
	
	fn button_function(&mut self, button_id: u16) -> bool {
		todo!()
	}
}

/*
   def post_draw(self, surface: Surface, camera: Camera, origin: Vec2):
	   position = Vec2(self.name_box.size.x + self.text_width + 2 * INNER_MARGIN,
					   self.slots[0].size.y / 2) + Vec2(MARGIN)
	   draw_text(surface, TEXT_EQUAL, origin + position, 20,
				 "black", align="left", camera=camera, bold=True)

   def draw_button(self, surface: Surface, camera: Camera, origin: Vec2, hovered: bool, button_id: int):
	   position = self.button_position(button_id)
	   size = self.button_size(button_id)

	   match self.buttons[button_id]:
		   case "name_box":
			   self.name_box.draw(surface, camera, origin + position)
			   if hovered:
				   draw_rect(surface, camera, "black", origin + position, size, 1, 2)
		   case "choose_type":
			   color = darker(TYPE_COLOR, .7) if hovered else TYPE_COLOR
			   draw_rect(surface, camera, color, origin + position, size, border_radius=SMALL_RADIUS)

			   if self.type is None:
				   for i in range(-1, 2):
					   draw_rect(surface, camera, "black",
								 origin + position + size / 2 + Vec2(i * 4, 0) - Vec2(1), Vec2(2))
			   else:
				   draw_text(surface, self.type, origin + position + size / 2, 16, camera=camera)

   def button_function(self, button_id: int):
	   match self.buttons[button_id]:
		   case "name_box":
			   self.name_box.select()
			   return False
		   case "choose_type":
			   return False

   def always_draw_button(self, button_id: int) -> bool:
	   match self.buttons[button_id]:
		   case "name_box":
			   return True
		   case "choose_type":
			   return True

   @property
   def text_width(self) -> int:
	   return int(BT_TYPE_SIZE.x if self.type is None
				  else TEXT_TYPES_SIZES[TYPES.index(self.type)].x)

   def as_ASTNode(self) -> ASTNodeVariableAssignment:
	   name_text = self.name_box.text if self.name_box.text else "-"
	   return ASTNodeVariableAssignment(name_text, self.slots[0].as_AST())
*/

/*
TEXT_EQUAL: str = "="
TEXT_EQUAL_SIZE: Vec2 = Vec2(FONT_20.size(TEXT_EQUAL))

TEXT_TYPES_SIZES: list[Vec2] = [Vec2(FONT_20.size(text)) for text in TYPES]
TEXT_HEIGHT: int = FONT_20.get_height()

BT_TYPE_SIZE: Vec2 = Vec2(20, 16)

TYPE_COLOR: Color = change_color(COLOR, s_fonc=lambda _: .1, v_fonc=lambda _: .9)
 */
