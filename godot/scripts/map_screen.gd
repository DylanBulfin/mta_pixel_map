extends Control

var routes: Array[SubwayRoute]
var selected_arr: Array[bool]

func _ready() -> void:
	for route in ScheduleManager.curr_schedule.routes.values():
		var fname = str("res://assets/line_icons/", route.route_id, ".png")
		if ResourceLoader.exists(fname):
			var icon = load(fname)
			State.route_icons[route.route_id] = icon
			%Sidebar.add_icon_item(icon)
			
			%Sidebar.select(len(routes), false)
			
			routes.append(route)
			selected_arr.append(true)
	
	%Map.initialize(routes)


func _on_sidebar_multi_selected(index: int, selected: bool) -> void:
	selected_arr[index] = selected
	if %Map.route_nodes.has(routes[index].route_id):
		%Map.route_nodes[routes[index].route_id].visible = selected
	


func _on_select_all_button_pressed() -> void:
	for i: int in range(len(selected_arr)):
		if not %Sidebar.is_selected(i):
			%Sidebar.select(i, false)
		if %Map.route_nodes.has(routes[i].route_id):
			%Map.route_nodes[routes[i].route_id].visible = true
		selected_arr[i] = true


func _on_deselect_all_button_pressed() -> void:
	for i: int in range(len(selected_arr)):
		selected_arr[i] = false
		if %Map.route_nodes.has(routes[i].route_id):
			%Map.route_nodes[routes[i].route_id].visible = false
	%Sidebar.deselect_all()
