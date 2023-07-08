use crate::blocs::widgets::TextBox;
use crate::blocs::{Bloc, Skeleton};
use pg_sdl::color::Colors;
use std::collections::HashMap;
// Login admin labo
// name:     .\adminlabo
// password: Robotique2023

/// Bloc d’assignation de variable - la variable nommée sur le côté gauche du bloc
/// prend la valeur de ce que contient le slot du côté droit.
pub struct VariableAssignmentBloc {
	bloc: Skeleton,
	name: TextBox,
	// type: str | None
}

impl VariableAssignmentBloc {
	// const COLOR: Color = hsv_color(30, 0.6, 1.0);
	/*
	pub fn new(position: Point2<f64>, blocs: &HashMap<u32, Bloc>) -> Self {
		Self {
			bloc: Skeleton::new(Self::COLOR, position, vec![Slot::new(Self::COLOR, "value")], Vec::new(), blocs),
			name: TextBox::new(Slot::DEFAULT_SIZE, paler(Self::COLOR, 0.1), "name".to_string()),
		}
	}
	*/
	pub fn repr(&self, blocs: &HashMap<u32, Box<dyn Bloc>>) -> String {
		let name = if self.name.get_text().is_empty() { "-".to_string() } else { self.name.get_text() };
		format!("VariableAssignment({}: ? = {})", name, self.bloc.slots[0].repr(blocs))
	}
}

/*
   def get_size(self) -> Vec2:
	   width = max([slot.size.x for slot in self.slots]) +\
			   self.name_box.size.x + self.text_width + TEXT_EQUAL_SIZE.x + 3 * INNER_MARGIN
	   height = sum([slot.size.y for slot in self.slots]) + max(len(self.slots) - 1, 0) * INNER_MARGIN
	   return Vec2(width, height) + Vec2(2 * MARGIN)

   def slot_position(self, slot_id: int) -> Vec2:
	   position_x = self.name_box.size.x + self.text_width + TEXT_EQUAL_SIZE.x + 3 * INNER_MARGIN
	   position_y = sum([slot.size.y for slot in self.slots[:slot_id]]) + slot_id * INNER_MARGIN
	   return Vec2(position_x, position_y) + Vec2(MARGIN)

   def post_draw(self, surface: Surface, camera: Camera, origin: Vec2):
	   position = Vec2(self.name_box.size.x + self.text_width + 2 * INNER_MARGIN,
					   self.slots[0].size.y / 2) + Vec2(MARGIN)
	   draw_text(surface, TEXT_EQUAL, origin + position, 20,
				 "black", align="left", camera=camera, bold=True)

   def button_size(self, button_id: int) -> Vec2:
	   match self.buttons[button_id]:
		   case "name_box":
			   return self.name_box.size
		   case "choose_type":
			   return Vec2(self.text_width, BT_TYPE_SIZE.y)

   def button_position(self, button_id: int) -> Vec2:
	   match self.buttons[button_id]:
		   case "name_box":
			   return Vec2(MARGIN, (self.size.y - self.name_box.size.y) / 2)
		   case "choose_type":
			   return Vec2(MARGIN + self.name_box.size.x + INNER_MARGIN,
						   (self.size.y - BT_TYPE_SIZE.y) / 2)

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
