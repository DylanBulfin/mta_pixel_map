extends Node
func find_closest_points(target: Vector2, count: int = State.num_closest) -> Array[ReferencePoint]:
	var new_points: Array[ReferencePoint] = State.reference_points.duplicate()
	
	new_points.sort_custom(
		func(a: ReferencePoint, b: ReferencePoint) -> bool: 
			return a.latlong.distance_squared_to(target) < b.latlong.distance_squared_to(target)
	)
	
	return new_points.slice(0, count)

func estimate_pixel_pos(latlong: Vector2, count: int = State.num_closest) -> Vector2i:
	var closest = find_closest_points(latlong, count)
	var total_distance := 0.0
	
	for point in closest:
		var diff = latlong - point.latlong
		var dist = diff.length()
		
		total_distance += (1 / dist)

	var total := Vector2.ZERO
	for point in closest:
		var diff = latlong - point.latlong
		var dist = diff.length()
		
		total +=  (1 / dist) * (point.ppos + point.local_scale * (Consts.X_NORM * diff.x + Consts.Y_NORM * diff.y))
		
	var res := total / total_distance
	
	return Vector2i(round(res.x), round(res.y))

func draw_megapixel(ppos: Vector2i, image: Image, color: Color) -> void:
	for px in range(State.pixel_factor):
		for py in range(State.pixel_factor):
			var final_pos = (State.pixel_factor * ppos) + Vector2i(px, py)
			
			if final_pos.x >= 0 and final_pos.y >= 0 and final_pos.x < image.get_width() and final_pos.y < image.get_height():
				image.set_pixelv(final_pos, color)
			else:
				# If any pixel is out of bounds all will be, just abort
				return

func decode_date(date: String) -> ServiceDate:
	var year := int(date.substr(0, 4))
	var month := int(date.substr(4, 2))
	var day := int(date.substr(6, 2))
	
	return ServiceDate.create_from_date(year, month, day)

func days_in_month(month: int, year: int) -> int:
	match month:
		1,3,5,7,8,10,12: return 31
		4,6,9,11: return 30
		2:
			if year % 4 == 0 and (year % 100 != 0 or year % 400 == 0):
				return 29
			else: return 28
		_: return 0
