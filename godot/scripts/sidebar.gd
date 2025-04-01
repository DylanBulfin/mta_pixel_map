extends ItemList

var routes: Array[SubwayRoute]
var selected: Array[bool]

func _ready() -> void:
	for route in ScheduleManager.curr_schedule.routes.values():
		var fname = str("res://assets/line_icons/", route.route_id, ".png")
		if ResourceLoader.exists(fname):
			var icon = load(fname)
			State.route_icons[route.route_id] = icon
			self.add_icon_item(icon)
			routes.append(route)
			selected.append(true)
			self.select(len(routes), false)
