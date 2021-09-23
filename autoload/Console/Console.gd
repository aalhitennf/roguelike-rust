extends CanvasLayer

onready var main_node: VBoxContainer = $Splitter 
onready var input_node: LineEdit = $Splitter/Background/Container/Input
onready var output_node: TextEdit = $Splitter/Background/Container/Output

var chandler = preload("res://autoload/Console/Chandler.gd").new()

var history = [""]
var current_line = history.size()

func _ready() -> void:
	chandler.tree = get_tree()
	chandler.output = funcref(self, "write_line")
	chandler.print_info()

func _input(event) -> void:

	if Input.is_action_just_pressed("toggle_console"):
		main_node.visible = not main_node.visible

		if main_node.visible:
			disable_other_inputs()
			input_node.grab_focus()
			input_node.clear()
		else:
			enable_other_inputs()

	elif event is InputEventKey and event.is_pressed():
		if !main_node.visible:
			return
		if event.scancode == KEY_UP:
			goto_history(-1)
		if event.scancode == KEY_DOWN:
			goto_history(1)

func goto_history(offset: int) -> void:
	current_line = clamp(current_line + offset, 0, history.size() - 1)
	input_node.text = history[current_line]
	input_node.call("set_cursor_position", 9999)

func write_line(value) -> void:
	output_node.text = str(
		output_node.text,
		"" if not output_node.text else "\n",
		value)
	output_node.set_v_scroll(9999999)

func _on_Input_text_entered(new_text) -> void:
	if !main_node.visible || new_text == "":
		return
	input_node.clear()
	chandler.process_command(new_text)
	history.append(new_text)
	current_line = history.size()

func disable_other_inputs() -> void:
	get_tree().current_scene.set_process_input(false)

func enable_other_inputs() -> void:
	get_tree().current_scene.set_process_input(true)

func register_command(command: String, description: String, ref: FuncRef, args: Array) -> void:
	chandler.register(command, description, ref)
