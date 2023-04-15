"""Ce programme python contient des fonctions pour interagir avec le monde vu par la caméra."""
from pygame import SRCALPHA, Surface, Vector2 as Vec2, Color, Rect, draw, transform
from MyPygameLibrary.Camera import Camera


def draw_arrow(surface: Surface, color: Color, point1: Vec2, point2: Vec2, width: int = 0,
               fixed_width: int = None):
	delta = point2 - point1
	length = delta.length()
	if fixed_width is None:
		poly = [(-.1, .8), (-.02, .8), (-.02, 0), (.02, 0), (.02, .8), (.1, .8), (0, 1)]
		poly = [point1 + Vec2(p).rotate(-delta.angle_to((0, 1))) * length for p in poly]
		draw.polygon(surface, color, poly, width)
	else:
		if length < fixed_width:
			draw.circle(surface, color, point1, fixed_width, width)
		else:
			poly = [
			  (fixed_width * 2.5, length - fixed_width * 5), (0, length),
			  (-fixed_width * 2.5, length - fixed_width * 5)
			]
			if length - fixed_width * 5 > 0:
				poly.extend([
				  (-fixed_width / 2, length - fixed_width * 5),
				  (-fixed_width / 2, 0), (fixed_width / 2, 0),
				  (fixed_width / 2, length - fixed_width * 5),
				])
			poly = [point1 + Vec2(p).rotate(-delta.angle_to((0, 1))) for p in poly]
			draw.polygon(surface, color, poly, width)


def draw_image(surface: Surface, camera: Camera, image: Surface, position: Vec2, size: float = 1):
	"""Affiche une image du point de vue de la caméra."""
	position = Vec2(position)
	size = Vec2(size)
	if camera.sees_rect(position, Vec2(image.get_size()) * size):
		surface.blit(transform.scale(image, (Vec2(image.get_size()) * camera.scale * size)),
		             camera.world2screen(position))


def draw_rect(surface: Surface, camera: Camera, color: Color, position: Vec2, size: Vec2,
              width: int = 0, border_radius: int = -1,
              border_top_left_radius: int = -1, border_top_right_radius: int = -1,
              border_bottom_left_radius: int = -1, border_bottom_right_radius: int = -1,
              alpha: int = None):
	"""Affiche un rectangle du point de vue de la caméra."""
	color = Color(color)
	position = Vec2(position) if position is not None else None
	size = Vec2(size)
	if camera.sees_rect(position, size):
		rect = Rect(camera.world2screen(position), size * camera.scale)
		if alpha is not None:
			rect = Rect(rect)
			surf = Surface(rect.size, SRCALPHA)
			color.a = alpha
			draw.rect(
			  surf, color, ((0, 0), rect.size),
			  int(width * camera.scale + 0.99),
			  int(border_radius * camera.scale + 0.99) if border_radius >= 0 else -1,
			  int(border_top_left_radius * camera.scale + 0.99) if border_top_left_radius >= 0 else -1,
			  int(border_top_right_radius * camera.scale + 0.99) if border_top_right_radius >= 0 else -1,
			  int(border_bottom_left_radius * camera.scale + 0.99) if border_bottom_left_radius >= 0 else -1,
			  int(border_bottom_right_radius * camera.scale + 0.99) if border_bottom_right_radius >= 0 else -1)
			surface.blit(surf, rect.topleft)
		else:
			draw.rect(
			  surface, color, rect,
			  int(width * camera.scale + 0.99),
			  int(border_radius * camera.scale + 0.99) if border_radius >= 0 else -1,
			  int(border_top_left_radius * camera.scale + 0.99) if border_top_left_radius >= 0 else -1,
			  int(border_top_right_radius * camera.scale + 0.99) if border_top_right_radius >= 0 else -1,
			  int(border_bottom_left_radius * camera.scale + 0.99) if border_bottom_left_radius >= 0 else -1,
			  int(border_bottom_right_radius * camera.scale + 0.99) if border_bottom_right_radius >= 0 else -1)


def draw_circle(surface: Surface, camera: Camera, color: Color, position: Vec2, radius: float,
                width: int = 0):
	"""Affiche un cercle du point de vue de la caméra."""
	position = Vec2(position)
	if camera.sees_rect(position - Vec2(radius), Vec2(radius * 2)):
		draw.circle(surface, color, camera.world2screen(position), radius * camera.scale,
		            int(width * camera.scale + 0.99))


def draw_poly(surface: Surface, camera: Camera, color: Color, vertices: list[Vec2],
              width: int = 0):
	"""Affiche un polygone du point de vue de la caméra."""
	left = min([vertex.x for vertex in vertices])
	right = max([vertex.x for vertex in vertices])
	bottom = min([vertex.y for vertex in vertices])
	top = max([vertex.y for vertex in vertices])
	if camera.sees_rect(Vec2(left, bottom), Vec2(right - left, top - bottom)):
		draw.polygon(surface, color,
		             [camera.world2screen(Vec2(0)) + vertex * camera.scale
		              for vertex in vertices], int(width * camera.scale + 0.99))


def draw_line(surface: Surface, camera: Camera, color: Color, point1: Vec2, point2: Vec2,
              width: int = 1):
	"""Affiche une ligne du point de vue de la caméra."""
	point1 = Vec2(point1)
	point2 = Vec2(point2)
	left = min([point1.x, point2.x])
	right = max([point1.x, point2.x])
	bottom = min([point1.y, point2.y])
	top = max([point1.y, point2.y])
	if camera.sees_rect(Vec2(left, bottom), Vec2(right - left, top - bottom)):
		draw.line(surface, color, camera.world2screen(point1), camera.world2screen(point2),
		          int(width * camera.scale + 0.99))

"""
def draw_arrow(surface: Surface, camera: Camera, color: Color, point1: Vec2, point2: Vec2):
	""""""Affiche une flèche du point de vue de la caméra.""""""
	delta = point1 - point2
	draw_line(surface, camera, color, point1, point1 - delta * 0.9, delta.length() / 25)
	draw_poly(surface, camera, color, [Vec2(point2.x, point2.y) +
	                                   point.rotate(Vec2().angle_to(-delta)) * delta.length()
	                                   for point in [Vec2(0), Vec2(-0.2, 0.08), Vec2(-0.2, -0.08)]])
"""