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
	
	var closest = find_closest_points(target)
	var total := Vector2.ZERO
	for point in closest:
		var diff = target - point.latlong
		
		total += point.ppos + point.local_scale * (x_norm * diff.x + y_norm * diff.y)
	
	print(total / 3)
	
	var image = preload("res://NYC.png")
	image.set_pixel(0, 0, Color.WEB_PURPLE)
	var image_texture = ImageTexture.create_from_image(image)
	
	%TextureRect.texture = image_texture
	

func find_closest_points(target: Vector2, count: int = num_nearby) -> Array[ReferencePoint]:
	var new_points = reference_points.duplicate()
	
	new_points.sort_custom(
		func(a: ReferencePoint, b: ReferencePoint): 
			return a.latlong.distance_squared_to(target) < b.latlong.distance_squared_to(target)
	)

	return new_points.slice(0, count)

func calculate_weight_avg(target: Vector2, closest_points: Array[ReferencePoint]) -> Vector2:
	var slopes: Array[float]
	var bs: Array[float]
	for point in closest_points:
		# Define a line that passes through this point and the target in latlong space
		var angle = point.latlong.angle_to_point(target)
		print(str(point.ppos, " ", angle * 180 / PI))
		
		var pixel_angle = angle + 29 * PI / 180 # Angle adjusted to pixel space
		print(str(point.ppos, " ", pixel_angle * 180 / PI)) 
		
		angle = angle
		
		var slope = tan(pixel_angle)
		# Formula: point.py = slope * point.px + b
		var b = point.py - slope * point.px
		print(str(-slope, " ", b))
		
		slopes.push_back(slope)
		bs.push_back(b)
	
	var intersections: Array[Vector2]
	
	# Calculate intersection points
	for i in range(0, num_nearby):
		var slope1 = slopes[i]
		var b1 = bs[i]
		for j in range(i, num_nearby):
			if i == j: continue
			var slope2 = slopes[j]
			var b2 = bs[j]
			
			var x = (b2 - b1) / (slope1 - slope2)
			var y = slope1 * x + b1
			
			print(str(Vector2(x, y), i, j))
			
			
		
	return Vector2.ZERO
		
