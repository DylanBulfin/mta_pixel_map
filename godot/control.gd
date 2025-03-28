extends Node2D

const nyc_angle: float = -29 * PI / 180

var reference_points: Array[ReferencePoint]
var num_nearby = 3

var x_norm = Vector2(cos(nyc_angle), sin(nyc_angle))
var y_norm = Vector2(sin(nyc_angle), -cos(nyc_angle))

func _ready() -> void:
	for point in preload("res://resources/data.tres").reference_points:
		reference_points.push_back(point)
	
	#var target = Vector2(-73.962303, 40.785911)
	#var target = Vector2(-74.045495, 40.689608)
	var target = Vector2(-73.972336, 40.729188)
	
	for point in reference_points:
		var closest = find_closest_points(point.latlong, 5)
		var local_scale_total := 0.0
		
		for other in closest:
			if other == point: continue
			var ll_distance = point.latlong.distance_to(other.latlong)
			var pix_distance = point.ppos.distance_to(other.ppos)
			local_scale_total += (pix_distance / ll_distance)
		
		point.local_scale = local_scale_total / len(closest)
	
	var image = preload("res://NYC.png")
	
	var shape_1 = get_shape(ShapeGodot.get_one_shape())
	var shape_6 = get_shape(ShapeGodot.get_six_shape())
	var shape_q = get_shape(ShapeGodot.get_q_shape())
	var shape_n = get_shape(ShapeGodot.get_n_shape())
	var shape_7 = get_shape(ShapeGodot.get_sev_shape())
	
	#draw_shape(shape_1, Color.RED, image)
	#draw_shape(shape_6, Color.WEB_GREEN, image)
	#draw_shape(shape_q, Color.YELLOW, image)
	#draw_shape(shape_n, Color.LIGHT_YELLOW, image)
	#draw_shape(shape_7, Color.PURPLE, image)
	var points = interpolate_points(shape_1)
	draw_points(points, Color.RED, image)
	
	var image_texture = ImageTexture.create_from_image(image)
	%TextureRect.texture = image_texture
		

func find_closest_points(target: Vector2, count: int = num_nearby) -> Array[ReferencePoint]:
	var new_points = reference_points.duplicate()
	
	new_points.sort_custom(
		func(a: ReferencePoint, b: ReferencePoint): 
			return a.latlong.distance_squared_to(target) < b.latlong.distance_squared_to(target)
	)

	return new_points.slice(0, count)

func estimate_ppos(target: Vector2, count: int = num_nearby):
	var closest = find_closest_points(target, count)
	
	var total := Vector2.ZERO
	for point in closest:
		var diff = target - point.latlong
		
		total += point.ppos + point.local_scale * (x_norm * diff.x + y_norm * diff.y)
	
	return total / count

func get_shape(json_string: String) -> ShapeGodot:
	var shape: ShapeGodot = ShapeGodot.new()
	var json: Dictionary = JSON.parse_string(json_string)
	
	for field: Dictionary in shape.get_property_list():
		var n = field.get("name")
		var u = field.get("usage")
		if n is String and u == PROPERTY_USAGE_NONE:
			if json[n] is Array:
				# Force type conversions
				shape[n].assign(json[n])
			else:
				shape[n] = json[n]

	return shape

func draw_shape(shape: ShapeGodot, color: Color, image: Image):
	for i in range(len(shape.lats)):
		var pos := Vector2(shape.lons[i], shape.lats[i])
		
		var estimate = estimate_ppos(pos, 3)
		print(estimate)
		
		image.set_pixel(round(estimate.x) as int, round(estimate.y) as int, color)

func draw_points(points: Array[Vector2], color: Color, image: Image):
	for point in points:
		image.set_pixel(round(point.x) as int,round(point.y) as int, color)

func interpolate_points(shape: ShapeGodot) -> Array[Vector2]:
	var old_points: Array[Vector2]
	var new_points: Array[Vector2]
	var all_points: Array[Vector2]
	for i in range(len(shape.lats)):
		var pos := Vector2(shape.lons[i], shape.lats[i])
		
		var estimate = estimate_ppos(pos, 3)
		old_points.append(estimate)
	
	all_points.append_array(old_points)
	
	for i in range(len(old_points) - 3):
		var p1 = old_points[i]
		var p2 = old_points[i + 1]
		var p3 = old_points[i + 2]
		var p4 = old_points[i + 3]
		
		var new_point = p2.cubic_interpolate(p3, p1, p4, 0.5)
		new_points.append(new_point)
	
	# TODO reiterate through new_points interpolating recursively\

	all_points.append_array(new_points)
	old_points = new_points.duplicate()
	new_points.clear()

	for _iteration in range(50):
		for i in range(len(old_points) - 3):
			var p1 = old_points[i]
			var p2 = old_points[i + 1]
			var p3 = old_points[i + 2]
			var p4 = old_points[i + 3]
			
			var new_point = p2.cubic_interpolate(p3, p1, p4, 0.5)
			new_points.append(new_point)
			
		all_points.append_array(new_points)
		old_points = new_points.duplicate()
		new_points.clear()
		
		print(len(all_points))

	return all_points
