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

# Points where we know latlong and pixel position. Used to estimate
# the position of others. Pixel positions relative to the original
# sprite size of the background image (220x286)
var references: Array[ReferencePoint]

func _ready() -> void:
	# In order to force type coercion
	references.assign(preload("res://resources/data.tres").reference_points)
	
	background_image = preload("res://NYC.png")
	var background_texture := ImageTexture.create_from_image(background_image)
	%BackgroundRect.texture = background_texture
	self.custom_minimum_size = Vector2(width, height)
	
	
