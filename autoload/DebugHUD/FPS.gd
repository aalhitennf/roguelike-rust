extends Label

func _ready():
	$Timer.connect("timeout", self, "update")

func update() -> void:
	self.set_text(str(Engine.get_frames_per_second()))

func toggle_timer() -> void:
	if $Timer.is_stopped():
		$Timer.start()
	else:
		$Timer.stop()
