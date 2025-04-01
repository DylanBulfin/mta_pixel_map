extends Control
class_name MapScene

var background_image: Image

var pixel_scale: int = 5

const BG_WIDTH_PX: int = 220
const BG_HEIGHT_PX: int = 286

var width: int:
	get: return BG_WIDTH_PX * pixel_scale
var height: int:
	get: return BG_HEIGHT_PX * pixel_scale

var route_nodes: Array[TextureRect]

# Points where we know latlong and pixel position. Used to estimate
# the position of others. Pixel positions relative to the original
# sprite size of the background image (220x286)
var references: Array[ReferencePoint]

var initialized: bool = false

func _ready() -> void:
	# In order to force type coercion
	references.assign(preload("res://resources/data.tres").reference_points)
	
	background_image = preload("res://NYC.png")
	var background_texture := ImageTexture.create_from_image(background_image)
	%BackgroundRect.texture = background_texture
	self.custom_minimum_size = Vector2(width, height)

func _process(delta: float) -> void:
	if not initialized: 
		initialized = true
		var shapes: Dictionary[SubwayShape, Variant]
		for trip: SubwayTrip in ScheduleManager.curr_schedule.trips.values():
			if trip.maybe_shape and not shapes.has(trip.maybe_shape):
				shapes[trip.maybe_shape] = null
				var color = trip.route.maybe_color
				if not color: color = Color.BLACK
				create_shape_route_node(trip.maybe_shape, color)
		var shapes_arr: Array[SubwayShape]
		shapes_arr.assign(shapes.values())
		
		

func create_shape_route_node(shape: SubwayShape, color: Color) -> void:
	var image = Image.create_empty(width, height, false, Image.FORMAT_RGBA8)
	var pposs: Array[Vector2i]
	for i in range(len(shape.lats)):
		var latlong = Vector2(shape.lons[i], shape.lats[i])
		var ppos = Utils.estimate_pixel_pos(latlong)
		pposs.append(ppos)
	
	pposs = interpolate_line(pposs)
	for ppos in pposs:
		Utils.draw_megapixel(ppos, image, color)
	
	var rect := TextureRect.new()
	rect.texture = ImageTexture.create_from_image(image)
	route_nodes.append(rect)
	%RouteContainer.add_child(rect)

# To allow easy switching between various screens
func clear_route_nodes() -> void:
	for rect in route_nodes:
		%RouteContainer.remove_child(rect)
	route_nodes.clear()

func interpolate_line(pposs: Array[Vector2i]) -> Array[Vector2i]:
	var i: int = 0
	
	var new_vals: Dictionary[Vector2i, Variant] # to approximate a set
	
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
		
		# Interpolate between first and last pair of points
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
