extends Control

func _ready() -> void:
	var image_texture = ImageTexture.create_from_image(State.background_image)
	%BackgroundRect.texture = image_texture

func draw_shape(shape: ShapeGodot, color: Color, image: Image):
	for i in range(len(shape.lats)):
		var pos := Vector2(shape.lons[i], shape.lats[i])
		
		var estimate = Utils.estimate_ppos(pos, State.num_closest)
		print(estimate)
		
		image.set_pixel(round(estimate.x) as int, round(estimate.y) as int, color)

func draw_points(points: Array[Vector2], color: Color, image: Image):
	for point in points:
		image.set_pixel(round(point.x) as int,round(point.y) as int, color)
