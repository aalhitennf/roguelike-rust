extends Camera2D

const zoom_step: float = 1.1
const min_zoom: float = 0.1
const max_zoom: float = 8.0
const pan_speed: int = 800

var _duration = 0.0
var _period_in_ms = 0.0
var _amplitude = 0.0
var _timer = 0.0
var _last_shook_timer = 0
var _previous_x = 0.0
var _previous_y = 0.0
var _last_offset = Vector2(0, 0)

## Rectangle used to limit camera panning.
## Note that the built in camera limits do not work: they don't actually constrain the position of the camera.
## They only stop the view from moving. For the player, this makes the camera appear to 'stick' at the edges of the map, 
## which is bad.
var limit_rect = null setget set_limit_rect
var mouse_captured: bool = false

#func _ready():
#	Console.register_command("shake", "Shake", funcref(self, "shake"), false)
#	if Globals.CameraPos == null or Globals.CameraZoom == null:
#		return
#	position = Globals.CameraPos
#	zoom = Globals.CameraZoom

func _unhandled_input(event)->void:
	if event.is_action_pressed("camera_zoom_in"):
		zoom /= zoom_step
		_snap_zoom_limits()
	if event.is_action_pressed("camera_zoom_out"):
		zoom *= zoom_step
		_snap_zoom_limits()
#	if event.is_action_pressed("camera_zoom_reset"):
#		focus_on_player()

	if event.is_action_pressed("mouse_left"):
		mouse_captured = true
	elif event.is_action_released("mouse_left"):
		mouse_captured = false

	if mouse_captured && event is InputEventMouseMotion:
		position -= event.relative * zoom #like we're grabbing the map


# use _process for smoother scrolling
func _process(delta) -> void:

	# Only shake when there's shake time remaining.
	if _timer == 0:
		return
	# Only shake on certain frames.
	_last_shook_timer = _last_shook_timer + delta
	# Be mathematically correct in the face of lag; usually only happens once.
	while _last_shook_timer >= _period_in_ms:
		_last_shook_timer = _last_shook_timer - _period_in_ms
		# Lerp between [amplitude] and 0.0 intensity based on remaining shake time.
		var intensity = _amplitude * (1 - ((_duration - _timer) / _duration))
		# Noise calculation logic from http://jonny.morrill.me/blog/view/14
		var new_x = rand_range(-1.0, 1.0)
		var x_component = intensity * (_previous_x + (delta * (new_x - _previous_x)))
		var new_y = rand_range(-1.0, 1.0)
		var y_component = intensity * (_previous_y + (delta * (new_y - _previous_y)))
		_previous_x = new_x
		_previous_y = new_y
		# Track how much we've moved the offset, as opposed to other effects.
		var new_offset = Vector2(x_component, y_component)
		set_offset(get_offset() - _last_offset + new_offset)
		_last_offset = new_offset
	# Reset the offset when we're done shaking.
	_timer = _timer - delta
	if _timer <= 0:
		_timer = 0
		set_offset(get_offset() - _last_offset)



	#smooth keyboard zoom
	if Input.is_action_pressed("camera_zoom_in"):
		zoom /= zoom_step
		
		_snap_zoom_limits()
	if Input.is_action_pressed("camera_zoom_out"):
		zoom *= zoom_step
		_snap_zoom_limits()
	
	var panning: Vector2 = Vector2(0, 0)
	if Input.is_action_pressed("camera_pan_up"):
		panning.y -= 1
	if Input.is_action_pressed("camera_pan_down"):
		panning.y += 1
	if Input.is_action_pressed("camera_pan_left"):
		panning.x -= 1
	if Input.is_action_pressed("camera_pan_right"):
		panning.x += 1
	
	if panning.length_squared() > 0:
		position += panning.normalized() * pan_speed * delta * zoom
		if limit_rect:
			_snap_to_limits()

# force position to be inside limit_rect
func _snap_to_limits()->void:
	position.x = clamp(position.x, limit_rect.position.x, limit_rect.end.x)
	position.y = clamp(position.y, limit_rect.position.y, limit_rect.end.y)

func _snap_zoom_limits()->void:
	zoom.x = clamp(zoom.x, min_zoom, max_zoom)
	zoom.y = clamp(zoom.y, min_zoom, max_zoom)

func set_limit_rect(rect)->void:
	limit_rect = rect
	_snap_to_limits()

func focus_on_player()->void:
	# zoom = Vector2(1.0, 1.0)
	position = Vector2(32, 32)


func shake(duration, frequency, amplitude):
	# Initialize variables.
	_duration = duration
	_timer = duration
	_period_in_ms = 1.0 / frequency
	_amplitude = amplitude
	_previous_x = rand_range(-1.0, 1.0)
	_previous_y = rand_range(-1.0, 1.0)
	# Reset previous offset, if any.
	set_offset(get_offset() - _last_offset)
	_last_offset = Vector2(0, 0)
