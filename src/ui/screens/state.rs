//! Metadata for a [screen](Screens).

use crossterm::event::{
	KeyEvent,
	MouseEvent,
};
use derive_new::new;
use ratatui::Frame;
use serde::{
	Deserialize,
	Serialize,
};

use crate::{
	event::Event,
	tui::FocusChange,
	ui::screens::Screen,
};

/// A set of properties that always goes with every instance of a [`Screen`].
#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenState {}

/// Wrapper struct for a [screen](Screens) and its [state](ScreenState).
#[derive(Debug, Serialize, Deserialize, new)]
pub struct ScreenHandle {
	pub screen: Screen,
	pub state: ScreenState,
}

/// Delegates methods [`Screen`] to [`ScreenHandle`], while
/// replacing the [`state`](ScreenState) parameter found on
/// those methods with its own [`ScreenHandle::state`] property. The return type
/// put at the end of each function declaration is also wrapped in a
/// [`crate::Result`].
///
/// For example...
/// ```ignore
/// delegate_screen! {
///     <vis> fn <name>(&mut self, <extra params>) -> <ok_type>;
/// }
/// ```
/// ...generates:
/// ```ignore
/// <vis> fn <name>(&mut self, <extra params>) -> crate::Result<<ok_type>> {
///     self.screen.<name>(&mut self.state, <extra params>)
/// }
/// ```
macro_rules! delegate_screen {
	{
		$(
			$(#[$meta:meta])*
			$vis:vis fn $fname:ident(
				&mut self$(,)*
				$($param:ident: $ty:ty),*
			) -> $ok_res:ty
		);+$(;)*
	} => {
		$(
			$(#[$meta])*
			$vis fn $fname(
				&mut self,
				$($param: $ty)*
			) -> $crate::Result<$ok_res> {
				self.screen.$fname(&mut self.state, $($param)*)
			}
		)+
	};
}

impl ScreenHandle {
	delegate_screen! {
		pub fn close(&mut self) -> ();
		pub fn event(&mut self, event: &Event) -> ();
		pub fn render(&mut self, frame: &mut Frame) -> ();
		pub fn key(&mut self, key: KeyEvent) -> ();
		pub fn mouse(&mut self, mouse: MouseEvent) -> ();
		pub fn paste(&mut self, text: String) -> ();
		pub fn focus(&mut self, change: FocusChange) -> ();
	}

	/// Constructs a new handle from a screen and initializes state from
	/// [`Screen::get_init_state`].
	pub fn from_screen(screen: Screen) -> Self {
		let state = screen.get_init_state();
		Self::new(screen, state)
	}
}
