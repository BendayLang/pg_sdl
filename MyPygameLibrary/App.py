"""Ce programme python contient la classe mère Application."""
from pygame import Color, RESIZABLE, Surface, Vector2 as Vec2, display, draw, event, init,\
	quit as pygame_quit
from pygame.time import Clock

from MyPygameLibrary.Inputs import Key, Inputs
from MyPygameLibrary.UI_elements import Button, Slider, darker, draw_text
from MyPygameLibrary.Camera import Camera
from MyPygameLibrary.World import draw_line, draw_rect


class App:
	LONG_CLICK_TIME: int = 300
	LONG_CLICK_SPEED: int = 0.02
	
	def __init__(self, name: str = "My game", color: Color = "sky blue",
	             size: tuple[int, int] = (1200, 750), fps: int = 60,
	             resizable: bool = True, quit_on_escape: bool = True,
	             bt_reset: bool = True, bt_quit: bool = True, draw_fps: bool = True):
		self.camera: Camera = Camera(self.window_size)
		
		self.quit_on_escape: bool = quit_on_escape
		self.bt_reset = bt_reset
		self.bt_quit = bt_quit
		self._draw_fps = draw_fps
		
		self.margin: int = 20
		size = Vec2(120, 50)
		self.ui_objects = {}
		if self.bt_quit:
			self.ui_objects["bt_quit"] = Button(
			  "indian red 2", self.mirror(Vec2(size.x + self.margin, self.margin), by_x=True),
			  size, text="QUIT")
		if self.bt_reset:
			self.ui_objects["bt_reset"] = Button(
			  "wheat 2", self.mirror(Vec2((size.x + self.margin) * 2, self.margin), by_x=True),
			  size, text="RESET")
	
	def run(self):
		"""Boucle principale de l’application."""
		while self.running:
			self.running = self.inputs.get_events(event.get())
			
			delta = self.clock.tick(self.fps)
			
			self.manage_inputs(delta)
			self.update(delta)
			
			if self.changed:
				self.window_surface.fill(self.window_color)
				self.draw_world()
				self.draw_ui()
			
			if self._draw_fps: self.draw_fps()
			display.flip()
			
			self.changed = False
		self.quit()
	
	def quit(self):
		"""Actions à effectuer quand on quitte l’application"""
		pygame_quit()
	
	def manage_inputs(self, delta: int):
		"""Gestion des entrées."""
		if self.quit_on_escape and self.inputs.K_ESCAPE == Key.PRESSED:
			self.running = False
			return
		
		if self.bt_quit:
			if self.ui_objects["bt_quit"].is_released():
				self.running = False
				return
		
		if self.bt_reset:
			if self.ui_objects["bt_reset"].is_released():
				self.reset()
		
		if self.inputs.WINDOW_RESIZED:
			self.resize()
		
		for ui_object in self.ui_objects.values():
			ui_object.update(delta, self.inputs, self.camera)
			if ui_object.changed:
				self.changed = True
		
		self.key_down_timer += delta
	
	def update(self, delta):
		"""Calculs de l'application."""
	
	def reset(self):
		"""Réinitialise l'application."""
	
	@property
	def window_size(self) -> Vec2: return Vec2(self.window_surface.get_size())
	
	def resize(self):
		self.changed = True
		self.camera.resize(self.window_size)
		
		if self.bt_quit:
			bt_quit = self.ui_objects["bt_quit"]
			bt_quit.position = self.mirror(Vec2(bt_quit.size.x + self.margin,
			                                    self.margin), by_x=True)
		if self.bt_reset:
			bt_reset = self.ui_objects["bt_reset"]
			bt_reset.position = self.mirror(Vec2((bt_reset.size.x + self.margin) * 2,
			                                     self.margin), by_x=True)
	
	def mirror(self, position: Vec2, by_x: bool = False, by_y: bool = False) -> Vec2:
		return Vec2(self.window_size.x - position.x if by_x else position.x,
		            self.window_size.y - position.y if by_y else position.y)
	
	def keys_move_slider(self, delta: int, slider: Slider,
	                     key_plus: Key, key_minus: Key, speed: float = None):
		if not slider.visible: return
		if speed is None: speed = self.LONG_CLICK_SPEED
		if key_plus == Key.PRESSED:
			slider.value += slider.clamp
			self.key_down_timer = 0
			self.changed = True
		elif key_minus == Key.PRESSED:
			slider.value -= slider.clamp
			self.key_down_timer = 0
			self.changed = True
		elif key_plus == Key.DOWN and self.key_down_timer > self.LONG_CLICK_TIME:
			slider.value += slider.clamp * int(delta * speed + 1)
			self.key_down_timer = self.LONG_CLICK_TIME
			self.changed = True
		elif key_minus == Key.DOWN and self.key_down_timer > self.LONG_CLICK_TIME:
			slider.value -= slider.clamp * int(delta * speed + 1)
			self.key_down_timer = self.LONG_CLICK_TIME
			self.changed = True
	
	def draw_world(self):
		"""Affiche les éléments à l'écran."""
	
	def draw_ui(self):
		"""Affiche les éléments d'interface utilisateur."""
		for ui_object in self.ui_objects.values():
			ui_object.draw(self.window_surface)
	
	def draw_fps(self):
		draw_text(self.window_surface, f"{self.clock.get_fps():.1f} FPS",
		          Vec2(100, 50), 30, "black", back_framed=True, framed=True)
	
	def draw_grid(self, color: Color = None, scale: float = 1 / 100, draw_border: bool = False):
		# Affiche la grille
		grid_color = darker(self.window_color, 0.9)\
			if color is None else color
		# scale = 10 ** round(log10(self.camera.scale) - 2)
		
		left, top = self.camera.left_top
		right, bottom = self.camera.right_bottom
		top2, bottom2 = int(top * scale), int(bottom * scale) + 1
		left2, right2 = int(left * scale), int(right * scale) + 1
		
		for y in range(top2, bottom2):  # Lignes
			draw_line(self.window_surface, self.camera, grid_color,
			          (left, y / scale), (right, y / scale), 1.2 if y % 5 else 4)
		for x in range(left2, right2):  # Colonnes
			draw_line(self.window_surface, self.camera, grid_color,
			          (x / scale, bottom), (x / scale, top), 1.2 if x % 5 else 4)
		
		if draw_border:
			limit_size = self.camera.right_bottom_limit - self.camera.left_top_limit
			draw_rect(self.window_surface, self.camera, "dark red",
			          self.camera.left_top_limit, limit_size, 12)
	
	def draw_clock(self):
		n = 12
		center = Vec2(100, 110)
		draw.circle(self.window_surface, "light grey", center, 30)
		[draw.line(self.window_surface, "black", center + Vec2(22, 0).rotate(a * 360 / n),
		           center + Vec2(28, 0).rotate(a * 360 / n), 2) for a in range(n)]
		draw.circle(self.window_surface, "black", center, 30, 2)
		draw.circle(self.window_surface, "red", center, 5)
		draw.line(self.window_surface, "red", center, center + Vec2(24, 0).rotate(self.rot), 3)
		self.rot += 360 / n
