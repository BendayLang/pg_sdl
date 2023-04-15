"""Ce programme python contient des éléments d’interface utilisateur."""
from colorsys import rgb_to_hsv, hsv_to_rgb
from copy import copy
from dataclasses import dataclass, field

from pygame import SRCALPHA, Surface, Vector2 as Vec2, Color, Rect, draw, transform
from pygame.font import SysFont

from MyPygameLibrary.Camera import Camera
from MyPygameLibrary.Inputs import Inputs, Key, Mouse
from MyPygameLibrary.World import draw_rect
from benday_rust import fuzzy_find

clip = lambda value, min_value, max_value: max(min(value, max_value), min_value)


@dataclass
class UiObject:
	
	def update(self, delta: int, inputs: Inputs, camera: Camera | None = None):
		"""Met à jour l’objet"""
	
	def draw(self, surface: Surface, camera: Camera | None = None, position: Vec2 | None = None):
		"""Affiche l’objet sur une surface donnée."""


class Button(UiObject):
	"""Objet graphique d’interface utilisateur récupérant une valeur booléenne."""
	
	def __init__(self, color: Color, position: Vec2, size: Vec2,
	             border: int = 2, border_color: Color = Color("black"),
	             text: str = None, text_size: int = 28, text_color: Color = Color("black"),
	             font: str = "arial", bold: bool = False, italic: bool = False,
	             visible: bool = True, corner_radius: int = 4):
		self.color = Color(color)
		self.position = position
		"""Position du centre du bouton."""
		self.size = size
		
		self._border = border
		self._border_color = border_color
		self._corner_radius = corner_radius
		self.state: Key = Key.UP
		self.hovered: bool = False
		
		self.text = str(text) if text is not None else None
		if self.text is not None:
			self._text_size = text_size
			self._text_color = text_color
			self._font = font
			self._bold = bold
			self._italic = italic
		
		self.changed: bool = visible
		"""Indique s'il y eu un changement d'état du bouton."""
		self.visible = visible
	
	def update(self, delta: int, inputs: Inputs, camera: Camera | None = None):
		"""Met à jour le bouton."""
		self.changed = False
		if not self.visible: return
		
		if self.state == Key.PRESSED:
			self.state = Key.DOWN
		if self.state == Key.RELEASED:
			self.state = Key.UP
		
		new_hovered = self.hit_box.collidepoint(inputs.mouse.position)
		if new_hovered != self.hovered:
			self.hovered = new_hovered
			self.changed = True
		
		if inputs.mouse.K_LEFT == Key.PRESSED and self.hovered:
			self.state = Key.PRESSED
			self.changed = True
		elif self.state == Key.DOWN and inputs.mouse.K_LEFT == Key.RELEASED:
			self.state = Key.RELEASED
			self.changed = True
	
	def draw(self, surface: Surface, camera: Camera | None = None, position: Vec2 | None = None):
		"""Affiche le bouton."""
		if not self.visible: return
		
		color = darker(self.color, .8) if self.hovered else self.color
		if self.state in [Key.DOWN, Key.PRESSED]:
			color = darker(self.color, .8 * .7)
		
		draw.rect(surface, color, self.hit_box, False, self._corner_radius)
		draw.rect(surface, self._border_color, self.hit_box, self._border, self._corner_radius)
		
		if self.text:
			draw_text(surface, self.text, self.hit_box.center, self._text_size, self._text_color,
			          self._font, self._bold, self._italic)
	
	@property
	def hit_box(self): return Rect(self.position, self.size)
	
	def is_pressed(self) -> bool: return self.state == Key.PRESSED
	
	def is_released(self) -> bool: return self.state == Key.RELEASED
	
	@property
	def border(self) -> int: return self._border
	
	@border.setter
	def border(self, value: int): self._border = value; self.changed = True
	
	@property
	def border_color(self) -> Color: return self._border_color
	
	@border_color.setter
	def border_color(self, value: Color): self._border_color = value; self.changed = True


class Slider(UiObject):
	"""Objet graphique d'interface utilisateur permettant de
	sélectionner une valeur dans une plage de valeurs."""
	
	def __init__(self, position: Vec2, size: Vec2, color: Color, span: tuple[int, int],
	             value: int = None, clamp: int = 1, dot_color: Color = Color("light grey"),
	             border: int = 2, border_color: Color = Color("black"),
	             visible: bool = True, **kwargs):
		self.position = position
		"""Position du centre du slider."""
		self.size = size
		self.color = Color(color)
		self.span = span
		"""Bornes minimum et maximum pour la valeur du slider. Ex: span = (-1, 1) -> value in (-1, 0, 1)"""
		self.clamp = clamp
		"""Arrondi pour lequel la valeur du slider va se fixer. Ex: clamp = 2 -> value in (0, 2, 4, 6, 8, 10)"""
		self._raw_value = (value - span[0]) / (span[1] - span[0]) if value is not None else 0.0
		self._last_value = self.value
		
		self._border = border
		self._border_color = border_color
		self._dot_color = dot_color
		self._other_color: Color = kwargs.get("other_color",
		                                      change_color(self.color, v_fonc=lambda v: 0.7 * v))
		self.state = Key.UP
		
		self.changed: bool = visible
		"""Indique s'il y eu un changement d'état du bouton."""
		self.visible = visible
	
	def update(self, delta: int, inputs: Inputs, camera: Camera | None = None):
		"""Met à jour le slider."""
		if not self.visible: return
		
		if self.state == Key.PRESSED:
			self.state = Key.DOWN
		if self.state == Key.RELEASED:
			self.state = Key.UP
		
		self.changed = False
		if inputs.mouse.K_LEFT == Key.PRESSED and self.hit_box.collidepoint(inputs.mouse.position):
			self.state = Key.PRESSED
			self.changed = True
		elif self.state == Key.DOWN and inputs.mouse.K_LEFT == Key.RELEASED:
			self.state = Key.RELEASED
			self.changed = True
		
		if self.state in [Key.PRESSED, Key.DOWN]:
			self._raw_value = self._orient((inputs.mouse.position - self._top_left) / self.length,
			                               inv_y=True).y
			self.value = self.value
	
	def draw(self, surface: Surface, camera: Camera | None = None, position: Vec2 | None = None):
		"""Affiche le slider."""
		if not self.visible: return
		show_hit_box = False
		
		draw.rect(surface, self.color, Rect(
		  self._top_left + (Vec2(-self._radius, 0) if self._horizontal
		                    else Vec2(0, (1 - self._raw_value) * self.length)),
		  self._orient(Vec2(self.width, self._raw_value * self.length + self._radius))),
		          0, self._radius)
		draw.rect(surface, self._other_color, Rect(
		  self._top_left + (Vec2(self._raw_value * self.length, 0) if self._horizontal
		                    else Vec2(0, -self._radius)),
		  self._orient(Vec2(self.width, (1 - self._raw_value) * self.length + self._radius))),
		          0, self._radius)
		draw.rect(surface, self._border_color, Rect(
		  self._top_left + self._orient(Vec2(0, -self._radius)),
		  self.size + self._orient(Vec2(0, self.width))), self._border, self._radius)
		
		if self.state in [Key.UP, Key.RELEASED]:
			draw.circle(surface, self._dot_color, self.dot_position, self._dot_radius)
		else:
			draw.circle(surface, self._pushed_color, self.dot_position, self._dot_radius)
		draw.circle(surface, self._border_color, self.dot_position, self._dot_radius, self._border)
		
		if show_hit_box:
			draw.rect(surface, self._border_color, self.hit_box, self._border)
	
	@property
	def value(self) -> float:
		return self.span[0] + round(
		  self._raw_value * (self.span[1] - self.span[0]) / self.clamp) * self.clamp
	
	@value.setter
	def value(self, value: float):
		self._raw_value = max(min((value - self.span[0]) / (self.span[1] - self.span[0]), 1), 0)
		if self._last_value != self.value:
			self._last_value = self.value
			self.changed = True
	
	@property
	def hit_box(self): return Rect(self._top_left - Vec2(20), self.size + Vec2(40))
	
	@property
	def _horizontal(self): return True if self.size.x > self.size.y else False
	
	@property
	def width(self) -> float: return self.size.y if self._horizontal else self.size.x
	
	@property
	def length(self) -> float: return self.size.x if self._horizontal else self.size.y
	
	def _orient(self, p: Vec2, inv_y=False) -> Vec2:
		return p.yx if self._horizontal else Vec2(p.x, 1 - p.y if inv_y else p.y)
	
	@property
	def _top_left(self) -> Vec2: return self.position - self.size / 2
	
	@property
	def _radius(self) -> int: return int(self.width / 2)
	
	@property
	def dot_position(self) -> Vec2:
		return self.position + self._orient(Vec2(0, (self._raw_value - 0.5) * self.length), True)
	
	@property
	def _dot_radius(self) -> int: return 10 + self._radius
	
	@property
	def _pushed_color(self) -> Color: return change_color(self._dot_color, v_fonc=lambda v: 0.7 * v)
	
	@property
	def border(self) -> int: return self._border
	
	@border.setter
	def border(self, value: int): self._border = value; self.changed = True
	
	@property
	def border_color(self) -> Color: return self._border_color
	
	@border_color.setter
	def border_color(self, value: Color): self._border_color = value; self.changed = True


DOT_RADIUS = 0.3


class CircularPad:
	"""Objet graphique d’interface utilisateur récupérant un Vecteur2."""
	
	def __init__(self, position: Vec2, radius: int, color: Color,
	             dot_color: Color, border: int = 3, visible: bool = True, **kwargs):
		self.position = position
		"""Position du centre du pad."""
		self.radius = radius
		self.color = Color(color)
		self.color.a = 127
		self.dot_color = Color(dot_color)
		
		self._border = border
		self._border_color = kwargs.get("border_color",
		                                change_color(self.color, v_fonc=lambda v: 0.7 * v))
		self._dot_border_color = kwargs.get("dot_border_color",
		                                    change_color(self.dot_color, v_fonc=lambda v: 0.7 * v))
		self.state: Key = Key.UP
		self.value = None
		
		self.changed: bool = visible
		"""Indique s'il y eu un changement d'état du pad."""
		self.visible = visible
	
	def update(self, mouse: Mouse):
		"""Met à jour le pad."""
		if not self.visible: return
		
		if self.state == Key.PRESSED:
			self.state = Key.DOWN
		if self.state == Key.RELEASED:
			self.state = Key.UP
		
		self.changed = False
		if mouse.K_LEFT == Key.PRESSED and self.contains(mouse.position):
			self.state = Key.PRESSED
			self.changed = True
		elif self.state == Key.DOWN and mouse.K_LEFT == Key.RELEASED:
			self.state = Key.RELEASED
			self.value = None
			self.changed = True
		
		if self.state in [Key.PRESSED, Key.DOWN]:
			self.value = (mouse.position - self.position) / (self.radius - self.dot_radius / 2)
			self.value = self.value.reflect((0, 1))
			if self.value.length() > 1: self.value = self.value.normalize()
			self.changed = True
	
	def draw(self, surface: Surface):
		"""Affiche le pad."""
		if not self.visible: return
		
		temp_surface = Surface(Vec2(self.radius * 2), SRCALPHA)
		draw.circle(temp_surface, self.color, Vec2(self.radius), self.radius)
		surface.blit(temp_surface, self.position - Vec2(self.radius))
		draw.circle(surface, self._border_color, self.position, self.radius, self._border)
		
		if self.value is not None:
			dot_position = self.position + self.value.reflect((0, 1)) * (self.radius - self.dot_radius)
			draw.circle(surface, self.dot_color, dot_position, self.dot_radius)
			draw.circle(surface, self._dot_border_color, dot_position, self.dot_radius, self._border)
	
	@property
	def dot_radius(self) -> int: return int(self.radius * DOT_RADIUS)
	
	def contains(self, point: Vec2) -> bool:
		return (point - self.position).length() <= self.radius + self.dot_radius


BAR_BLINK_TIME: int = 400
KEY_DOWN_TIME: int = 400
KEY_DOWN_SPEED: float = 0.3
MARGIN: int = 5


class TextBox(UiObject):
	"""Objet graphique d’interface utilisateur permettant d’entrer du texte."""
	
	def __init__(self, position: Vec2, size: Vec2, color: Color = "white", default_color: Color = None,
	             text: str = "", default_text: str = "enter text", fixed_size: bool = True,
	             border: int = 2, border_color: Color = "black", selected: bool = False,
	             corner_radius: int = 0, text_size: int = 20, text_color: Color = "black",
	             font: str = "arial", bold: bool = False, italic: bool = False, visible: bool = True):
		self.position = position
		"""Position du centre de la boîte de texte."""
		self.size = size.copy()
		self._default_size = size.copy()
		self.fixed_size = fixed_size
		self.color = Color(color)
		
		self._border = border
		self._border_color = Color(border_color)
		self.corner_radius = corner_radius
		
		self.text = str(text)
		self._default_text = str(default_text)
		self._default_color = Color(default_color) if default_color is not None else self.color
		self._text_size = text_size
		self._text_color = Color(text_color)
		self._font = font
		self._bold = bold
		self._italic = italic
		
		self._timer_bar_blink = 0
		self.char = len(self.text)
		self._timer_key_down = 0
		
		self.size_changed: bool = False
		self.changed: bool = False
		"""Indique s'il y eu un changement d'état de la boîte de texte."""
		self.selected: bool = selected
		"""Indique si la boîte de texte est sélectionnée et prend les entrées du clavier."""
		self.visible: bool = visible
		
		self.text_changed: bool = False
		self.clicked_outside: bool = False
		
		self.hovered = False
		self.text_surface: Surface = None
	
	def __repr__(self):
		return f"TextBox({self.text})"
	
	def __deepcopy__(self, memodict=None):
		the_copy = copy(self)
		the_copy.size = self.size.copy()
		return the_copy
	
	def update(self, delta: int, inputs: Inputs, camera: Camera | None = None):
		"""Met à jour la boîte de texte."""
		self.changed = False
		self.clicked_outside = False
		self.text_changed = False
		self.size_changed = False
		
		if not self.visible: return
		
		if inputs.mouse.K_LEFT == Key.PRESSED:
			if self.hit_box.collidepoint(inputs.mouse.position):
				self.selected = True
				self.changed = True
			else:
				self.clicked_outside = True
		
		if not self.selected: return
		
		self._timer_key_down += delta
		self._timer_bar_blink += delta
		
		if inputs.TEXT_INPUT:
			self.text = self.text[:self.char] + inputs.TEXT_INPUT + self.text[self.char:]
			self.char += len(inputs.TEXT_INPUT)
			self._timer_bar_blink = 0
			self.changed = True
			self.text_changed = True
		
		if self.char > 0:
			if inputs.K_BACKSPACE == Key.PRESSED:
				self.text = self.text[:self.char - 1] + self.text[self.char:]
				self.char -= 1
			elif inputs.K_BACKSPACE == Key.DOWN and self._timer_key_down > KEY_DOWN_TIME:
				self.text = self.text[:self.char - 1] + self.text[self.char:]
				self.char -= 1
			
			if inputs.K_LEFT == Key.PRESSED:
				self.char -= 1
			elif inputs.K_LEFT == Key.DOWN and self._timer_key_down > KEY_DOWN_TIME:
				self.char -= 1
		
		if self.char < len(self.text):
			if inputs.K_DELETE == Key.PRESSED:
				self.text = self.text[:self.char] + self.text[self.char + 1:]
				self.text_changed = True
			elif inputs.K_DELETE == Key.DOWN and self._timer_key_down > KEY_DOWN_TIME:
				self.text = self.text[:self.char] + self.text[self.char + 1:]
				self.text_changed = True
			
			if inputs.K_RIGHT == Key.PRESSED:
				self.char += 1
			elif inputs.K_RIGHT == Key.DOWN and self._timer_key_down > KEY_DOWN_TIME:
				self.char += 1
		
		if Key.PRESSED in [inputs.K_BACKSPACE, inputs.K_DELETE, inputs.K_LEFT, inputs.K_RIGHT]:
			self._timer_bar_blink = 0
			self._timer_key_down = 0
			self.changed = True
			self.text_changed = True
		elif Key.DOWN in [inputs.K_BACKSPACE, inputs.K_DELETE, inputs.K_LEFT, inputs.K_RIGHT]:
			self._timer_bar_blink = 0
			if self._timer_key_down > KEY_DOWN_TIME:
				self._timer_key_down = KEY_DOWN_TIME - delta / KEY_DOWN_SPEED
				self.changed = True
		
		if BAR_BLINK_TIME < self._timer_bar_blink < BAR_BLINK_TIME + delta:
			self.changed = True
		if self._timer_bar_blink > BAR_BLINK_TIME * 2:
			self._timer_bar_blink = 0
			self.changed = True
		
		if inputs.K_RETURN == Key.PRESSED:
			self.unselect()
		
		if not self.fixed_size:
			self.update_size(camera)
	
	def update_size(self, camera: Camera | None = None):
		margin = MARGIN if camera is None else MARGIN * camera.scale
		font_size = self._text_size if camera is None else int(self._text_size * camera.scale)
		font = SysFont(self._font, font_size, self._bold, self._italic)
		text_surface = font.render(self.text, False, "black")
		text_width = text_surface.get_width()
		width = text_width if camera is None else int(text_width / camera.scale)
		
		self.size.x = max(self._default_size.x, width + 2 * margin)
		self.size_changed = True
	
	def draw_background(self, surface: Surface, camera: Camera | None, position: Vec2):
		if self.selected:
			color = "white"
		elif self.hovered:
			color = darker(self._default_color, 0.8)
		elif self.text:
			color = self.color
		else:
			color = self._default_color
		
		if camera is None:
			rect = Rect(position, self.size)
			draw.rect(surface, color, rect, 0, self.corner_radius)
			if self._border: draw.rect(surface, self._border_color, rect,
			                           self._border, self.corner_radius)
		else:
			draw_rect(surface, camera, color, position, self.size, 0, self.corner_radius)
			if self._border: draw_rect(surface, camera, self._border_color, position, self.size,
			                           self._border, self.corner_radius)
	
	def get_text_surface(self, camera: Camera | None) -> Surface:
		margin = MARGIN if camera is None else MARGIN * camera.scale
		size = self.size if camera is None else self.size * camera.scale
		
		font_size = self._text_size if camera is None else int(self._text_size * camera.scale)
		font = SysFont(self._font, font_size, self._bold, self._italic)
		
		if self.hovered:
			color = change_color(self._default_color, s_fonc=lambda s: s * .3, v_fonc=lambda v: .3)
		else:
			color = change_color(self._text_color, s_fonc=lambda s: s, v_fonc=lambda v: v)
		
		left_text_surface = font.render(self.text[:self.char], True, color)
		left_text = left_text_surface.get_rect()
		left_text.centery = size.y / 2
		
		right_text_surface = font.render(self.text[self.char:], True, color)
		right_text = right_text_surface.get_rect()
		right_text.centery = size.y / 2
		
		if left_text.width + right_text.width < size.x - margin * 2 or not self.selected:
			left_text.left = margin
			right_text.left = left_text.right + 1
		elif right_text.width < size.x - margin * 2:
			right_text.right = size.x - margin
			left_text.right = right_text.left - 1
		else:
			left_text.right = margin
			right_text.left = left_text.right + 1
		
		surf = Surface(size, SRCALPHA)
		surf.blit(left_text_surface, left_text)
		surf.blit(right_text_surface, right_text)
		
		if self.selected and BAR_BLINK_TIME > self._timer_bar_blink:
			draw.line(surf, self._text_color,
			          (left_text.right, (size.y - font_size) / 2),
			          (left_text.right, (size.y + font_size) / 2), 1)
		
		return surf
	
	def get_default_text_surface(self, camera: Camera | None) -> Surface:
		margin = MARGIN if camera is None else MARGIN * camera.scale
		size = self.size if camera is None else self.size * camera.scale
		
		font_size = self._text_size if camera is None else int(self._text_size * camera.scale)
		font = SysFont(self._font, font_size, self._bold, self._italic)
		
		if self.hovered:
			color = change_color(self._default_color, s_fonc=lambda s: s * .3, v_fonc=lambda v: .3)
		else:
			color = change_color(self._text_color, s_fonc=lambda s: s / 2, v_fonc=lambda v: .4)
		
		text_surface = font.render(self._default_text, True, color)
		text_rect = text_surface.get_rect()
		text_rect.centery = size.y / 2
		
		text_rect.left = margin
		surf = Surface(size, SRCALPHA)
		surf.blit(text_surface, text_rect)
		
		if self.selected and BAR_BLINK_TIME > self._timer_bar_blink:
			draw.line(surf, self._text_color,
			          (margin, (size.y - font_size) / 2),
			          (margin, (size.y + font_size) / 2), 1)
		
		return surf
	
	def draw(self, surface: Surface, camera: Camera | None = None, position: Vec2 | None = None):
		"""Affiche la boîte de texte."""
		if not self.visible: return
		if self.position is not None: position = self.position
		
		self.draw_background(surface, camera, position)
		
		if self.changed or self.text_surface is None\
		  or (camera.size_changed if camera is not None else False):
			self.text_surface = self.get_text_surface(camera) if self.text\
				else self.get_default_text_surface(camera)
		
		if camera is not None: position = camera.world2screen(position)
		surface.blit(self.text_surface, position)
	
	@property
	def valid_input(self) -> bool: return bool(self.text)
	
	def select(self):
		self.selected = True
		self.changed = True
		self.char = len(self.text)
		self._timer_bar_blink = 0
	
	def unselect(self):
		self.selected = False
		self.changed = True
		self.char = len(self.text)
		self._timer_bar_blink = 0
	
	@property
	def hit_box(self):
		return Rect(Vec2(0, 0) if self.position is None else self.position, self.size)
	
	@property
	def border(self) -> int:
		return self._border
	
	@border.setter
	def border(self, value: int):
		self._border = value
		self.changed = True
	
	@property
	def border_color(self) -> Color:
		return self._border_color
	
	@border_color.setter
	def border_color(self, value: Color):
		self._border_color = value
		self.changed = True


class MultiBox:
	"""Groupe de boîtes de texte."""
	
	def __init__(self, position: Vec2, size: Vec2, array: tuple[int, int], text: list[list[str]] = None,
	             selected_box: tuple[int, int] = (0, 0),
	             color: Color = Color("white"), border: int = 2, border_color: Color = Color("black"),
	             corner_radius: int = 0, text_size: int = 25, text_color: Color = Color("black"),
	             font: str = "arial", bold: bool = False, italic: bool = False, visible: bool = True):
		self.text_boxes = [[
		  TextBox(position + size.elementwise() * Vec2(x + (1 - array[0]) / 2, y + (1 - array[1]) / 2),
		          size, text[x][y] if text is not None else "", color, border, border_color, False,
		          corner_radius, text_size, text_color, font, bold, italic)
		  for y in range(array[1])] for x in range(array[0])]
		
		self.array = array
		self.selected_box = selected_box
		self.visible = visible
	
	def update(self, tick: float, inputs: Inputs):
		if not self.visible: return
		
		for line in self.text_boxes:
			for text_box in line:
				text_box.update(tick, inputs)
		
		if inputs.K_TAB == Key.PRESSED and self.selected_box:
			if inputs.K_SHIFT == Key.DOWN and self.selected_box != (0, 0):
				if self.selected_box[0] == 0:
					self.selected_box = (self.array[0] - 1, self.selected_box[1] - 1)
				else:
					self.selected_box = (self.selected_box[0] - 1, self.selected_box[1])
			
			elif inputs.K_SHIFT != Key.DOWN and self.selected_box != (
			  self.array[0] - 1, self.array[1] - 1):
				if self.selected_box[0] == self.array[0] - 1:
					self.selected_box = (0, self.selected_box[1] + 1)
				else:
					self.selected_box = (self.selected_box[0] + 1, self.selected_box[1])
	
	def draw(self, surface: Surface):
		if not self.visible: return
		
		for line in self.text_boxes:
			for text_box in line:
				text_box.draw(surface)
	
	@property
	def changed(self) -> bool:
		temp = []
		for line in self.text_boxes:
			for text_box in line:
				temp.append(text_box.changed)
		return any(temp)
	
	@property
	def valid_input(self) -> bool: return all(all(text_line) for text_line in self.text_array)
	
	@property
	def selected_box(self) -> tuple[int, int]:
		for x, line in enumerate(self.text_boxes):
			for y, text_box in enumerate(line):
				if text_box.selected: return x, y
	
	@selected_box.setter
	def selected_box(self, value: tuple[int, int]):
		for line in self.text_boxes:
			for text_box in line:
				text_box.selected = False
		self.text_boxes[value[0]][value[1]].select()
	
	@property
	def hit_box(self) -> Rect:
		return Rect(self.box(0, 0).hit_box.topleft, self.box(0, 0).size.elementwise() * self.array)
	
	def box(self, x: int, y: int) -> TextBox: return self.text_boxes[x][y]
	
	@property
	def text_array(self) -> list[list[str]]:
		return [[text_box.text for text_box in line] for line in self.text_boxes]
	
	def text(self, x: int, y: int) -> str: return self.text_array[x][y]


SLIDER_WIDTH: int = 12

LONG_CLICK_TIME: int = 300
LONG_CLICK_SPEED: int = 0.01


@dataclass(slots=True)
class RollingList(UiObject):
	"""Liste déroulante permettant de choisir parmi une liste d’items."""
	position: Vec2
	height: int
	base_words: list[str]
	color: Color = field(default_factory=lambda: Color("white"))
	selected_word: int | None = None
	text_color: Color = field(default_factory=lambda: Color("black"))
	text_size: int = 20
	font: str = "arial"
	border: int = 1
	border_color: Color = field(default_factory=lambda: Color("black"))
	corner_radius: int = 0
	slider_position: int = 0
	slider_selected: bool = False
	slider_color: Color = None
	text_surface: Surface = None
	line_height: int = None
	words: list[str] = None
	size: Vec2 = None
	hovered_word: int | None = None
	slider_hovered: bool = False
	clicked_outside: bool = False
	confirm_selection: bool = False
	key_down_timer: int = 0
	changed: bool = False
	
	def __post_init__(self):
		if self.slider_color is None:
			self.slider_color = darker(self.color, .7)
		self.words = self.base_words
		
		font = SysFont(self.font, self.text_size)
		width = max([font.size(word)[0] for word in self.words])
		self.size = Vec2(width + SLIDER_WIDTH + 3 * MARGIN, self.height)
		
		self.line_height = font.get_height()
		self.text_surface = Surface((width, len(self.words) * self.line_height), SRCALPHA)
		for i, text in enumerate(self.words):
			height = i * self.line_height
			self.text_surface.blit(font.render(text, True, self.text_color), (0, height))
	
	def update(self, delta: int, inputs: Inputs, camera: Camera | None = None):
		self.changed = False
		self.clicked_outside = False
		self.confirm_selection = False
		if not self.words: return
		
		self.key_change_word(delta, inputs)
		
		position = inputs.mouse.position - self.position
		size = Vec2(self.size.x, min(self.text_surface.get_height(), self.size.y))
		
		new_hovered_word = self.collide_word(inputs.mouse.position)
		if new_hovered_word != self.hovered_word:
			self.hovered_word = new_hovered_word
			self.changed = True
		
		new_slider_hovered = 0 < size.x - position.x < SLIDER_WIDTH + MARGIN and 0 < position.y < size.y
		if new_slider_hovered != self.slider_hovered:
			self.slider_hovered = new_slider_hovered
			self.changed = True
		
		if self.slider_selected:
			delta_y = inputs.mouse.delta.y
			if delta_y:
				self.slider_position += delta_y
				self.slider_position = clip(self.slider_position, 0, self.course)
				self.changed = True
			if inputs.mouse.K_LEFT == Key.RELEASED:
				self.slider_selected = False
				self.changed = True
		
		if not (0 < position.x < size.x and 0 < position.y < size.y):
			if inputs.mouse.K_LEFT == Key.PRESSED:
				self.clicked_outside = True
				if self.selected_word is not None:
					self.selected_word = None
					self.changed = True
			return
		
		if inputs.mouse.K_LEFT == Key.PRESSED:
			if self.hovered_word is not None:
				if self.hovered_word == self.selected_word:
					self.confirm_selection = True
				self.selected_word = self.hovered_word
				self.changed = True
			else:
				self.slider_position = (position.y - self.slider_height / 2)\
				                       / (size.y - self.slider_height) * self.course
				self.slider_position = clip(self.slider_position, 0, self.course)
				self.slider_selected = True
			self.changed = True
		
		
		if inputs.mouse.scroll:
			if self.text_surface.get_height() > self.size.y:
				self.slider_position -= inputs.mouse.scroll * 5
				self.slider_position = clip(self.slider_position, 0, self.course)
			self.changed = True
	
	def draw(self, surface: Surface, camera: Camera | None = None, position: Vec2 | None = None):
		if not self.words: return
		
		size = Vec2(self.size.x, min(self.text_surface.get_height(), self.size.y))
		box_surface = Surface(size, SRCALPHA)
		draw.rect(box_surface, self.color, ((0, 0), size), 0, self.corner_radius)
		
		width = size.x - 2 * MARGIN - SLIDER_WIDTH
		if self.hovered_word is not None:
			draw.rect(box_surface, darker(self.color, .9),
			          ((1, self.hovered_word * self.line_height - self.slider_position),
			           (width, self.line_height)))
		
		if self.selected_word is not None:
			color = darker(self.color, .7 * .9) if self.hovered_word == self.selected_word\
				else darker(self.color, .7)
			draw.rect(box_surface, color,
			          ((1, self.selected_word * self.line_height - self.slider_position),
			           (width, self.line_height)))
		
		draw.rect(box_surface, self.border_color, ((0, 0), size), self.border, self.corner_radius)
		
		for i, text in enumerate(self.words):
			height = i * self.line_height - self.slider_position
			draw.line(box_surface, self.text_color, (0, height), (width, height))
		
		box_surface.blit(self.text_surface, (MARGIN, -self.slider_position))
		
		# Slider
		color = darker(self.slider_color, .9) if self.slider_hovered else self.slider_color
		if self.slider_selected: color = darker(color, .7)
		draw.rect(box_surface, color, (Vec2(width, self.slider_position) + Vec2(MARGIN),
		                               (SLIDER_WIDTH, self.slider_height)), 0, self.corner_radius)
		draw.rect(box_surface, "black", (Vec2(width, self.slider_position) + Vec2(MARGIN),
		                                 (SLIDER_WIDTH, self.slider_height)), 1, self.corner_radius)
		
		surface.blit(box_surface, self.position)
	
	def key_change_word(self, delta: int, inputs: Inputs):
		self.key_down_timer += delta
		delta_time = self.key_down_timer - LONG_CLICK_TIME
		
		if self.selected_word is None:
			if inputs.K_DOWN == Key.PRESSED:
				self.selected_word = 0
				self.slider_position = 0
				self.key_down_timer = 0
				self.changed = True
			elif inputs.K_UP == Key.PRESSED:
				self.selected_word = len(self.words) - 1
				self.slider_position = self.course
				self.key_down_timer = 0
				self.changed = True
			return
		
		if inputs.K_DOWN == Key.PRESSED:
			self.selected_word += 1
			self.key_down_timer = 0
			if self.selected_word > len(self.words) - 1:
				self.selected_word = 0
				self.slider_position = 0
			elif self.selected_word + 2.4 > (self.size.y + self.slider_position) / self.line_height:
				self.slider_position = (self.selected_word + 2.4) * self.line_height - self.size.y
				self.slider_position = clip(self.slider_position, 0, self.course)
			self.changed = True
		
		elif inputs.K_UP == Key.PRESSED:
			self.selected_word -= 1
			self.key_down_timer = 0
			if self.selected_word < 0:
				self.selected_word = len(self.words) - 1
				self.slider_position = self.course
			elif self.selected_word - 2 < (self.size.y + self.slider_position) / self.line_height:
				self.slider_position = (self.selected_word + 3.8) * self.line_height - self.size.y
				self.slider_position = clip(self.slider_position, 0, self.course)
			self.changed = True
		
		elif inputs.K_DOWN == Key.DOWN and self.key_down_timer > LONG_CLICK_TIME:
			if delta_time * LONG_CLICK_SPEED >= 1:
				self.selected_word += int(delta_time * LONG_CLICK_SPEED)
				self.key_down_timer = LONG_CLICK_TIME
				if self.selected_word > len(self.words) - 1:
					self.selected_word = len(self.words) - 1
					self.slider_position = self.course
					self.key_down_timer = 0
				elif self.selected_word + 2.4 > (self.size.y + self.slider_position) / self.line_height:
					self.slider_position = (self.selected_word + 2.4) * self.line_height - self.size.y
					self.slider_position = clip(self.slider_position, 0, self.course)
				self.changed = True
		
		elif inputs.K_UP == Key.DOWN and self.key_down_timer > LONG_CLICK_TIME:
			if delta_time * LONG_CLICK_SPEED >= 1:
				self.selected_word -= int(delta_time * LONG_CLICK_SPEED)
				self.key_down_timer = LONG_CLICK_TIME
			
			if self.selected_word < 0:
				self.selected_word = 0
				self.slider_position = 0
				self.key_down_timer = 0
			elif self.selected_word - 2 < (self.size.y + self.slider_position) / self.line_height:
				self.slider_position = (self.selected_word + 3.8) * self.line_height - self.size.y
				self.slider_position = clip(self.slider_position, 0, self.course)
			self.changed = True
	
	def collide_word(self, point: Vec2) -> int | None:
		"""Renvoie l’index du mot avec lequel collisionne le point."""
		position = point - self.position
		size = Vec2(self.size.x, min(self.text_surface.get_height(), self.size.y))
		
		if not (0 < position.x < size.x - SLIDER_WIDTH - MARGIN and 0 < position.y < size.y):
			return None
		return int((position.y + self.slider_position) / self.line_height)
	
	def update_words(self, search_word: str):
		self.words = fuzzy_find(self.base_words, search_word)
		self.selected_word = None
		self.slider_position = 0
		
		font = SysFont(self.font, self.text_size)
		width = self.size.x - 3 * MARGIN - SLIDER_WIDTH
		self.text_surface = Surface((width, len(self.words) * self.line_height), SRCALPHA)
		for i, text in enumerate(self.words):
			height = i * self.line_height
			self.text_surface.blit(font.render(text, True, self.text_color), (0, height))
		
		self.changed = True
	
	@property
	def course(self) -> int:
		return max(self.text_surface.get_height() - self.size.y, 0)
	
	@property
	def slider_height(self) -> int:
		if self.text_surface.get_height() <= self.size.y:
			return self.text_surface.get_height() - 2 * MARGIN
		return self.size.y - self.course - 2 * MARGIN
	
	@property
	def selected_text(self) -> str:
		if self.selected_word is None: return None
		return self.words[self.selected_word]


def draw_text(surface: Surface, text: str, position: Vec2, size: int = 20, color: Color = "black",
              font: str = "tw cen", bold: bool = False, italic: bool = False,
              align: str = "center", camera: Camera = None,
              back_framed: bool = False, framed: bool = False, contoured: bool = False, **kwargs):
	"""Cette fonction permet d'afficher du texte."""
	if camera:
		position = camera.world2screen(position)
		size *= camera.scale
	text = str(text)
	color = Color(color)
	font = SysFont(font, int(size), bold, italic)
	text_surface = font.render(text, True, color)
	rect = text_surface.get_rect()
	if align == "left":
		rect.midleft = position
	elif align == "center":
		rect.center = position
	elif align == "right":
		rect.midright = position
	
	if camera:
		if not camera.sees_rect(camera.screen2world(
		  Vec2(rect.bottomleft)), Vec2(rect.size) / camera.scale): return
	
	if back_framed:
		back_frame_color = kwargs.get("back_frame_color",
		                              change_color(color, v_fonc=lambda v: 1 - v))
		back_frame_rect = rect.copy()
		back_frame_rect.size += Vec2(0.5, 0.25) * size
		back_frame_rect.center -= Vec2(0.5, 0.25) * size / 2
		back_frame_radius = int(size / 4)
		draw.rect(surface, back_frame_color, back_frame_rect, False, back_frame_radius)
	
	if framed:
		frame_color = kwargs.get("frame_color", color)
		frame_width = kwargs.get("frame_width", int(size / 16 + 0.99))
		frame_rect = rect.copy()
		frame_rect.size += Vec2(0.5, 0.25) * size
		frame_rect.center -= Vec2(0.5, 0.25) * size / 2
		frame_radius = int(size / 4)
		draw.rect(surface, frame_color, frame_rect, frame_width, frame_radius)
	
	if contoured:
		contour_color = kwargs.get("contour_color", change_color(color, v_fonc=lambda v: 1 - v))
		contour_surface = font.render(text, True, contour_color)
		for pos in [Vec2(x, y) for y in [-1, 0, 1] for x in [-1, 0, 1] if x or y]:
			contour_rect = rect.copy()
			contour_rect.center += Vec2(pos) * size / 20
			surface.blit(contour_surface, contour_rect)
	
	surface.blit(text_surface, rect)


def do_nothing(x: float) -> float: return x


def hsv_color(h: int, s: int, v: int) -> Color:
	"""Renvoie une couleur selon la teinte, la saturation et la luminosité."""
	r, g, b = hsv_to_rgb(clip(h / 360 % 1, 0, 1),
	                     clip(s / 100, 0, 1),
	                     clip(v / 100, 0, 1))
	return Color(int(r * 255), int(g * 255), int(b * 255))


def change_color(color: Color, h_fonc=do_nothing, s_fonc=do_nothing, v_fonc=do_nothing) -> Color:
	if type(color) is not Color: color = Color(color)
	h, s, v = rgb_to_hsv(color.r / 255, color.g / 255, color.b / 255)
	r, g, b = hsv_to_rgb(clip(h_fonc(h), 0, 1),
	                     clip(s_fonc(s), 0, 1),
	                     clip(v_fonc(v), 0, 1))
	return Color(int(r * 255), int(g * 255), int(b * 255))


def darker(color: Color, value_change: float = .6) -> Color:
	return change_color(color, v_fonc=lambda v: value_change * v)


def draw_ellipsis(surface: Surface, color: str,
                  p: Vec2, q: Vec2,
                  origin: Vec2 = Vec2(0, 0), width: int = 0):
	"""Affiche une ellipse dans un parallélogramme basé sur deux vecteurs.
	La méthode La construction de l’axe de Rytzsche.
	https://de.wikipedia.org/wiki/Rytzsche_Achsenkonstruktion"""
	n = (p.rotate(90) + q) / 2
	q_n = q - n
	
	q_b = q_n.copy()
	q_b.scale_to_length(n.length() - q_n.length())
	b = q + q_b
	
	rect = Rect(0, 0, q_b.length(), n.length() + q_n.length())
	angle = b.angle_to((0, 1))
	
	surf = Surface((abs(rect.width), abs(rect.height)), SRCALPHA)
	draw.ellipse(surf, color, rect, width)
	surf = transform.rotate(surf, angle)
	surface.blit(surf, origin - Vec2(surf.get_size()) / 2)
