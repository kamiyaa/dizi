pub const DEFAULT_KEYMAP: &str = "
mapcommand = [
	{ keys = [ \"q\" ],		command = \"close\" },
	{ keys = [ \"Q\" ],		command = \"/server/quit\" },

	{ keys = [ \"r\" ],		command = \"reload_dirlist\" },
	{ keys = [ \"z\", \"h\" ],		command = \"toggle_hidden\" },

	# arrow keys
	{ keys = [ \"arrow_up\" ],	command = \"cursor_move_up\" },
	{ keys = [ \"arrow_down\" ],	command = \"cursor_move_down\" },
	{ keys = [ \"arrow_left\" ],	command = \"cd ..\" },
	{ keys = [ \"arrow_right\" ],	command = \"open\" },
	{ keys = [ \"\n\" ],		command = \"open\" },
	{ keys = [ \"end\" ],		command = \"cursor_move_end\" },
	{ keys = [ \"home\" ],		command = \"cursor_move_home\" },
	{ keys = [ \"page_up\" ],		command = \"cursor_move_page_up\" },
	{ keys = [ \"page_down\" ],	command = \"cursor_move_page_down\" },

	{ keys = [ \"c\", \"d\" ],		command = \":cd \" },

	{ keys = [ \" \" ],		command = \"/player/toggle/play\" },
	{ keys = [ \"0\" ],		command = \"/player/volume/increase 1\" },
	{ keys = [ \"9\" ],		command = \"/player/volume/decrease 1\" },
	{ keys = [ \"S\" ],		command = \"/player/toggle/shuffle\" },
	{ keys = [ \"R\" ],		command = \"/player/toggle/repeat\" },
	{ keys = [ \"N\" ],		command = \"/player/toggle/next\" },
	{ keys = [ \"n\" ],		command = \"/player/play/next\" },
	{ keys = [ \"p\" ],		command = \"/player/play/previous\" },

	{ keys = [ \"t\" ],		command = \"select --all=true --toggle=true\" },

	{ keys = [ \":\" ],		command = \":\" },
	{ keys = [ \";\" ],		command = \":\" },

	{ keys = [ \"/\" ],		command = \":search \" },
	{ keys = [ \"\\\" ],		command = \":search_glob \" },
	{ keys = [ \"C\" ],		command = \"search_skim\" },

	{ keys = [ \"n\" ],		command = \"search_next\" },
	{ keys = [ \"N\" ],		command = \"search_prev\" },

	{ keys = [ \"s\", \"r\" ],		command = \"sort reverse\" },
	{ keys = [ \"s\", \"l\" ],		command = \"sort lexical\" },
	{ keys = [ \"s\", \"m\" ],		command = \"sort mtime\" },
	{ keys = [ \"s\", \"n\" ],		command = \"sort natural\" },
	{ keys = [ \"s\", \"s\" ],		command = \"sort size\" },
	{ keys = [ \"s\", \"e\" ],		command = \"sort ext\" },

	{ keys = [ \"g\", \"r\" ],		command = \"cd /\" },
	{ keys = [ \"g\", \"c\" ],		command = \"cd ~/.config\" },
	{ keys = [ \"g\", \"d\" ],		command = \"cd ~/Downloads\" },
	{ keys = [ \"g\", \"e\" ],		command = \"cd /etc\" },
	{ keys = [ \"g\", \"h\" ],		command = \"cd ~/\" },
]
";
