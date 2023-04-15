"""Ce programme python contient une caméra virtuelle 2D."""
from pygame import Vector2 as Vec2

from MyPygameLibrary import Inputs
from MyPygameLibrary.Inputs import Key


class Camera:
	"""Objet virtuel permettant de se déplacer et de zoomer dans un monde."""
	
	def __init__(self, resolution: Vec2, position: Vec2 = None, scale: float = 1,
	             mouse_keys=None, vertical_scroll: bool = False,
	             zoom_speed: float = 2 ** (1 / 4), zoom_on_mouse: bool = True,
	             min_scale: float = None, max_scale: float = None,
	             left_limit: float = None, right_limit: float = None,
	             top_limit: float = None, bottom_limit: float = None):
		self._resolution = resolution
		self.position: Vec2 = -resolution / scale / 2 if position is None else position
		"""Position de la caméra dans le monde."""
		self.scale: float = scale
		"""Échelle où niveau de zoom de la caméra."""
		
		self._mouse_keys: list[int] = [1] if mouse_keys is None else mouse_keys
		self._vertical_scroll: bool = vertical_scroll
		
		self._zoom_on_mouse: bool = zoom_on_mouse
		self._min_scale: float = min_scale
		self._max_scale: float = max_scale
		
		self._zoom_speed: float = zoom_speed
		"""Échelle à laquelle la caméra est agrandie à chaque scroll."""
		
		self._left_limit: float | None = left_limit
		self._right_limit: float | None = right_limit
		self._top_limit: float | None = top_limit
		self._bottom_limit: float | None = bottom_limit
		
		self.changed: bool = True
		"""Indique s'il y eu un changement de position ou de taille de la caméra."""
		self.size_changed: bool = False
		"""Indique s'il y eu un changement de taille de la caméra."""
	
	def update(self, input: Inputs):
		"""Met à jour la caméra."""
		self.changed = False
		self.size_changed = False
		
		if 1 in self._mouse_keys and input.mouse.K_LEFT == Key.DOWN:
			self.move(-input.mouse.delta / self.scale)
		elif 2 in self._mouse_keys and input.mouse.K_WHEEL == Key.DOWN:
			self.move(-input.mouse.delta / self.scale)
		elif 3 in self._mouse_keys and input.mouse.K_RIGHT == Key.DOWN:
			self.move(-input.mouse.delta / self.scale)
		
		if input.mouse.scroll:
			if self._vertical_scroll:
				if input.K_CONTROL == Key.DOWN:
					center = input.mouse.position if self._zoom_on_mouse else self._resolution / 2
					self.zoom(input.mouse.scroll, center)
				else:
					self.move(Vec2(0, -20 * input.mouse.scroll / self.scale))
			else:
				center = input.mouse.position if self._zoom_on_mouse else self._resolution / 2
				self.zoom(input.mouse.scroll, center)
	
	def resize(self, new_size: Vec2):
		self.move((self._resolution - new_size) / self.scale / 2)
		self._resolution = new_size
		self.changed = True
	
	def move(self, delta: Vec2):
		"""Déplace la caméra."""
		if not delta: return
		self.changed = True
		
		self.position += delta
		
		left, top = self.left_top
		right, bottom = self.right_bottom
		
		if left < self._left_limit if self._left_limit is not None else False:
			self.position.x = self._left_limit
		if right > self._right_limit if self._right_limit is not None else False:
			self.position.x = self._right_limit - self._resolution.x / self.scale
		
		if top < self._top_limit if self._top_limit is not None else False:
			self.position.y = self._top_limit
		if bottom > self._bottom_limit if self._bottom_limit is not None else False:
			self.position.y = self._bottom_limit - self._resolution.y / self.scale
	
	def zoom(self, zoom_level: float, center: Vec2):
		"""Zoom la caméra."""
		if not zoom_level: return
		self.changed = True
		self.size_changed = True
		scale = self._zoom_speed ** zoom_level
		
		if self._min_scale > self.scale * scale if self._min_scale else False:
			self.move(center / self.scale * (1 - self.scale / self._min_scale))
			self.scale = self._min_scale
		elif self._max_scale < self.scale * scale if self._max_scale else False:
			self.move(center / self.scale * (1 - self.scale / self._max_scale))
			self.scale = self._max_scale
		else:
			self.move(center / self.scale * (1 - 1 / scale))
			self.scale *= scale
	
	def screen2world(self, point: Vec2) -> Vec2:
		"""Renvoie la position d'un point à l'écran en position dans le monde."""
		return Vec2(point) / self.scale + self.position
	
	def world2screen(self, point: Vec2) -> Vec2:
		"""Renvoie la position d'un point dans le monde en position à l'écran."""
		return (Vec2(point) - self.position) * self.scale
	
	@property
	def left_top(self) -> Vec2: return self.screen2world(Vec2(0))
	
	@property
	def right_bottom(self) -> Vec2: return self.screen2world(self._resolution)
	
	@property
	def left_top_limit(self) -> Vec2: return Vec2(self._left_limit, self._top_limit)
	
	@property
	def right_bottom_limit(self) -> Vec2: return Vec2(self._right_limit, self._bottom_limit)
	
	@property
	def size(self) -> Vec2:
		return self.right_bottom - self.left_top
	
	def sees_point(self, point: Vec2) -> bool:
		"""Indique si un point est visible dans le champ de la caméra."""
		left, top = self.left_top
		right, bottom = self.right_bottom
		return left < point.x < right and bottom < point.y < top
	
	def sees_rect(self, position: Vec2, size: Vec2) -> bool:
		"""Indique si un rectangle est visible dans le champ de la caméra."""
		left, top = self.left_top
		right, bottom = self.right_bottom
		return left - size.x < position.x < right and top - size.y < position.y < bottom
	
	@property
	def aspect_ratio(self) -> float: return self._resolution.x / self._resolution.y
	
	@property
	def center(self) -> Vec2: return self.position + self.size / 2
	
	@center.setter
	def center(self, value: Vec2): self.position = value - self.size / 2
