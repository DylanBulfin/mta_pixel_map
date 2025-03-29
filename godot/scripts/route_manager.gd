extends Node

var rects: Array[TextureRect]

func _ready() -> void:
	for route: Route in State.routes:
		route.image = Image.create_empty(
			State.background_image.get_width(), 
			State.background_image.get_height(),
			false,
			Image.FORMAT_RGBA8
		)
		# Draw a shape for the route
		var shape: ShapeGodot = State.shapes[route.default_shape_full_name]
		if not shape:
			push_error("Unable to find default shape for route ", route.name)
			
		var pposs: Array[Vector2i]
		
		for i: int in range(len(shape.lats)):
			var latlong := Vector2(shape.lons[i], shape.lats[i])
			
			var ppos := Utils.estimate_pixel_pos(latlong)
			
			if ppos.x > 0 and ppos.y > 0:
				pposs.append(ppos)
		
		
		for ppos in interpolate_line(pposs):
			for px in range(State.pixel_factor):
				for py in range(State.pixel_factor):
					route.image.set_pixelv((State.pixel_factor * ppos) + Vector2i(px, py), route.color)
		
		var rect = TextureRect.new();
		var texture = ImageTexture.create_from_image(route.image)
		rect.set_texture(texture)
		rects.append(rect)
		%RouteContainer.add_child(rect)

func set_visible(index: int, vis: bool) -> void:
	rects[index].visible = vis

func interpolate_line(pposs: Array[Vector2i]) -> Array[Vector2i]:
	var i: int = 0
	
	var new_vals: Dictionary#[Vector2i, null], to approximate a set
	
	while true:
		var pre_a = pposs[i]
		var a = pposs[i + 1]
		var b = pposs[i + 2]
		var post_b = pposs[i + 3]
		
		var x = 0.0
		while x <= 1.0:
			var val = Vector2(a).cubic_interpolate(Vector2(b), Vector2(pre_a), Vector2(post_b), x)
			var vi = Vector2i(round(val.x), round(val.y))
			
			if vi not in new_vals:
				new_vals[vi] = null
			
			x += 0.05
				
		# Explicitly do 1.0 as well
		var val = Vector2(a).cubic_interpolate(Vector2(b), Vector2(pre_a), Vector2(post_b), 1.0)
		var vi = Vector2i(round(val.x), round(val.y))
		
		if vi not in new_vals:
			print(i)
			new_vals[vi] = null
		
		i += 1
		
		if i + 3 >= len(pposs):
			break
	
	var res: Array[Vector2i]
	res.assign(new_vals.keys())

	return res
