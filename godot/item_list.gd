extends ItemList

func _ready() -> void:
	var count = 0
	
	for route in State.routes:
		self.add_item(route.name)
		self.select(count, false)
		
		count += 1
	
	self.add_item("Deselect All")
	self.add_item("Select All")

func _on_multi_selected(index: int, selected: bool) -> void:
	if index == len(State.routes):
		for item in self.get_selected_items():
			if item == index: continue
			%RouteManager.set_visible(item, false)
		deselect_all()
	elif index == len(State.routes) + 1:
		select_all()
		self.deselect(index)
	else:
		%RouteManager.set_visible(index, selected)

func select_all() -> void:
	for i in range(len(State.routes)):
		if not self.is_selected(i):
			self.select(i, false)
			%RouteManager.set_visible(i, true)
