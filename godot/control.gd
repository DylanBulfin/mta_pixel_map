extends Control

var reference_points: Array[ReferencePoint]

func _ready() -> void:
	for point in preload("res://resources/data.tres").reference_points:
		reference_points.push_back(point)
	
	var target = Vector2(40.785911, -73.962303)
	
	var avg = calculate_weight_avg(target, find_closest_points(target))
	
	print(avg)

func find_closest_points(target: Vector2) -> Array[ReferencePoint]:
	var new_points = reference_points.duplicate()
	
	new_points.sort_custom(
		func(a: ReferencePoint, b: ReferencePoint): 
			return a.latlong.distance_squared_to(target) < b.latlong.distance_squared_to(target)
	)

	return new_points.slice(0, 1)

func calculate_weight_avg(target: Vector2, closest_points: Array[ReferencePoint]) -> Vector2:
	for point in closest_points:
		# Define a line that passes through this point and the target in latlong space
		var angle = point.latlong.angle_to_point(target)
		print(str(point.ppos, " ", angle))
		print(angle * 180 / PI)
		
	return Vector2.ZERO
		
