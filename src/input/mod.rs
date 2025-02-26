pub mod input_handler;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Input {
	// meta key bindings
	Abort,
	ActionBreak,
	ActionDrop,
	ActionEdit,
	ActionFixup,
	ActionPick,
	ActionReword,
	ActionSquash,
	Edit,
	Escape,
	Exit,
	ForceAbort,
	ForceRebase,
	Help,
	InsertLine,
	Kill,
	MoveCursorDown,
	MoveCursorEnd,
	MoveCursorHome,
	MoveCursorLeft,
	MoveCursorPageDown,
	MoveCursorPageUp,
	MoveCursorRight,
	MoveCursorUp,
	No,
	OpenInEditor,
	Other,
	Rebase,
	Redo,
	ScrollBottom,
	ScrollDown,
	ScrollJumpDown,
	ScrollJumpUp,
	ScrollLeft,
	ScrollRight,
	ScrollTop,
	ScrollUp,
	ShowCommit,
	ShowDiff,
	SwapSelectedDown,
	SwapSelectedUp,
	ToggleVisualMode,
	Undo,
	Yes,

	// raw input values
	Backspace,
	BackTab,
	Character(char),
	Delete,
	Down,
	End,
	Enter,
	Home,
	Insert,
	Left,
	PageDown,
	PageUp,
	Resize,
	Right,
	Tab,
	Up,

	// some events should be
	Ignore,
}
