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
