extends CanvasLayer

var animation_disabled: bool = false
var active: bool
var thread = null

const TRANSITION_TIME: float = 0.5

func _ready() -> void:
	$Control.rect_size = OS.get_window_size()

func _thread_load(path: String) -> void:
	#print("Hello from thread")
	call_deferred("set_active", true)
	#print("Set active: ", active)
	call_deferred("show")
	#print("Layer: ", self.layer)
	call_deferred("animate_in")
	call_deferred("animate_in_noise")
	#print("Started animation in")
	yield($Tween, 'tween_completed')
	#print("Animation in done")
	var ril = ResourceLoader.load_interactive(path)
	#print("Resource loader created")
	if !ril:
		Console.write_line("[SceneSwitcher] Failed to load resource: " + path)
		call_deferred("_thread_done", false)
	else:
		var res = null
		while true:
			var err = ril.poll()
			if err == ERR_FILE_EOF:
				res = ril.get_resource()
				break
			elif err != OK:
				break
		call_deferred("_thread_done", res)
	#print("Thread done")

func _thread_done(resource) -> void:
	#print("Back in main thread")
	#print("Waiting for thread to finish...")
	# Always wait for threads to finish, this is required on Windows.
	thread.wait_to_finish()
	#print("Thread finished")
	if resource:
	# Instantiate new scene.
		var new_scene = resource.instance()
		#print("Created new scene instance")
		# Free current scene.
		get_tree().current_scene.free()
		get_tree().current_scene = null
		#print("Freed previous scene")
		# Add new one to root.
		get_tree().root.add_child(new_scene)
		#print("Added new scene to root")
		# Set as current scene.
		get_tree().current_scene = new_scene
		#print("New scene set as current scene")
	#progress.visible = false
	if not animation_disabled:
		animate_out()
		animate_out_noise()
		#print("Started animation out, waiting for it to complete")
		yield($Tween, 'tween_completed')
		#print("Animation out done")
		hide()
		#print("Layer: ", self.layer)
	set_active(false)
	#print("Set active: ", false)
	#print("All done")

func hide() -> void:
	self.layer = -1
	$Control.visible = false

func show() -> void:
	self.layer = 2
	$Control.visible = true

func set_active(state: bool) -> void:
	active = state

func load_scene(path: String) -> void:
	if active:
		#print_debug("Already loading")
		return
	thread = Thread.new()
	#print("Created new thread")
	thread.start(self, "_thread_load", path)
	#print("Thread started")

func animate_in() -> void:
	$Tween.interpolate_property(
		$Control/Shards.get_material(),
		"shader_param/cutoff",
		1.0, 0.0, TRANSITION_TIME, 
		Tween.TRANS_LINEAR,
		Tween.EASE_IN_OUT
	)
	$Tween.start()

func animate_out() -> void:
	$Tween.interpolate_property(
		$Control/Shards.get_material(),
		"shader_param/cutoff",
		0.0, 1.0, TRANSITION_TIME, 
		Tween.TRANS_LINEAR,
		Tween.EASE_IN_OUT
	)
	$Tween.start()
	
func animate_in_noise() -> void:
	$Tween.interpolate_property(
		$Control/Noise.get_material(),
		"shader_param/power",
		0.0, 0.05, TRANSITION_TIME, 
		Tween.TRANS_LINEAR,
		Tween.EASE_IN_OUT
	)
	$Tween.start()

func animate_out_noise() -> void:
	$Tween.interpolate_property(
		$Control/Noise.get_material(),
		"shader_param/power",
		0.05, 0.0, TRANSITION_TIME, 
		Tween.TRANS_LINEAR,
		Tween.EASE_IN_OUT
	)
	$Tween.start()
