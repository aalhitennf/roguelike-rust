extends Node2D

func move(dir: Vector2) -> void:
	var new_x = self.position.x + (dir.x * 16)
	var new_y = self.position.y + (dir.y * 16)
	self.position = Vector2(new_x, new_y)
	get_parent().get_node("Light").update_fog(self.position / 16)

func _input(_event) -> void:
		# Movement
	if Input.is_action_just_pressed('move_right'):
		move(Vector2(1, 0))
	if Input.is_action_just_pressed('move_left'):
		move(Vector2(-1, 0))
	if Input.is_action_just_pressed('move_up'):
		move(Vector2(0, -1))
	if Input.is_action_just_pressed('move_down'):
		move(Vector2(0, 1))
	#if Input.is_action_just_pressed('move_upleft'):
	#	move(Vector2(-1, -1))
	#if Input.is_action_just_pressed('move_upright'):
	#	move(Vector2(1, -1))
	#if Input.is_action_just_pressed('move_downleft'):
	#	move(Vector2(-1, 1))
	#if Input.is_action_just_pressed('move_downright'):
	#	move(Vector2(1, 1))
