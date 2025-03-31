extends Node

var rects: Array[TextureRect]

func _ready() -> void:
	init_routes()
	init_stations()
	
func init_stations() -> void:
	var image = Image.create_empty(State.background_image.get_width(), State.background_image.get_height(), false, Image.FORMAT_RGBA8)
	
	for station_id in State.stations:
		var station = State.stations[station_id]
		var ppos = Utils.estimate_pixel_pos(Vector2(station.lon, station.lat))
		Utils.draw_megapixel(ppos, image, Color.WHITE)
	
	var texture = ImageTexture.create_from_image(image)
	%StationRect.texture = texture

func init_routes() -> void:
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
			Utils.draw_megapixel(ppos, route.image, route.color)
		
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
		
		for weight in [
			0.00, 0.05, 0.10, 0.15, 0.20, 0.25, 0.30, 0.35, 0.40, 0.45,
			0.50, 0.55, 0.60, 0.65, 0.70, 0.75, 0.80, 0.85, 0.90, 0.95, 1.00
		]:
			var val = Vector2(a).cubic_interpolate(Vector2(b), Vector2(pre_a), Vector2(post_b), weight)
			var vi = Vector2i(round(val.x), round(val.y))
			
			if vi not in new_vals:
				new_vals[vi] = null
		
		i += 1
		
		if i + 3 >= len(pposs):
			break
	
	# Almost certainly not necessary, shape points are usually so close that many
	# of them correspond to the same pixel. This change would only be necessary if
	# there were a major gap between the first and second or the last and second
	# to last. But since I want to support arbitrary shapes instead of tailoring
	# too heavily to the default ones, I 
	for triple in [[2, 1, 0], [-3, -2, -1]]:
		var pre_a = pposs[triple[0]]
		var a = pposs[triple[1]]
		var b = pposs[triple[2]]
		
		for weight in [
			0.00, 0.05, 0.10, 0.15, 0.20, 0.25, 0.30, 0.35, 0.40, 0.45,
			0.50, 0.55, 0.60, 0.65, 0.70, 0.75, 0.80, 0.85, 0.90, 0.95, 1.00
		]:
			var val = Vector2(a).cubic_interpolate(Vector2(b), Vector2(pre_a), Vector2(b), weight)
			var vi = Vector2i(round(val.x), round(val.y))
			
			if vi not in new_vals:
				new_vals[vi] = null
	
	var res: Array[Vector2i]
	res.assign(new_vals.keys())

	return res
