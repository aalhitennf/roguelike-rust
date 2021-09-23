extends Node

var tree: SceneTree
var output: FuncRef
var commands: Dictionary = {}

func _init() -> void:
	register("list", "Show all available commands", funcref(self, "list"))
	register("exit", "Close the game", funcref(self, "exit"))

func process_command(expr: String) -> void:
	expr = expr.strip_edges(true, true)
	if expr == "": return
	var args = expr.to_lower().split(" ", false)
	var command = args[0]
	args.remove(0)
	run(command, args)

func register(command: String, description: String, ref: FuncRef) -> void:
	commands[command] = {
		"description": description,
		"ref": ref,
	}

func run(command: String, args: PoolStringArray) -> void:
	var cmd = commands.get(command)
	if !cmd:
		write_output("Invalid command: " + command)
		return
	cmd.ref.call_func(args)

func write_output(text) -> void:
	output.call_func(text)

func print_info() -> void:
	write_output("list - Show all available commands")

func exit() -> void:
	tree.quit()

func list() -> void:
	for cmd in commands.keys():
		var text = cmd + " - " + commands[cmd].description
		output.call_func(text)
