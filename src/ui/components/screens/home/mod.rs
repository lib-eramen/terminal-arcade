//! Home screen to greet a user on running.

use crossterm::event::KeyCode;
use ratatui::{
	layout::Rect,
	prelude::Buffer,
};

use self::banner::{
	Banner,
	BannerState,
	BANNER_ASSET_PATH,
};
use crate::{
	events::{
		Event,
		InputEvent,
		ScreenEvent,
	},
	services::files::AppFiles,
	ui::{
		screen::{
			handle::ScreenHandleData,
			metadata::ScreenMetadataBuilder,
			Screen,
		},
		UiElement,
	},
};

mod banner;

/// The home screen.
#[derive(Debug)]
pub struct HomeScreen<'a> {
	banner: Banner<'a>,
	banner_state: BannerState,
}

impl HomeScreen<'_> {
	/// Constructs a new home screen.
	pub fn new(app_files: &AppFiles) -> crate::Result<Self> {
		let banner_frames = Self::get_banner_frames(app_files)?;
		let original_banner = Self::get_original_banner(app_files)?;

		let banner_state =
			BannerState::new(original_banner, banner_frames.len());
		let banner = Banner::new(banner_frames);

		Ok(Self {
			banner,
			banner_state,
		})
	}

	/// Gets the banner frames that the banner animation uses.
	fn get_banner_frames(app_files: &AppFiles) -> crate::Result<Vec<String>> {
		static BANNER_FRAME_HEIGHT: usize = 14;
		Ok(app_files
			.read_string_asset(BANNER_ASSET_PATH.join("letter-frames.txt"))?
			.lines()
			.collect::<Vec<_>>()
			.chunks_exact(BANNER_FRAME_HEIGHT)
			.map(|chunk| chunk.join("\n"))
			.collect())
	}

	/// Gets the original banner frame.
	fn get_original_banner(app_files: &AppFiles) -> crate::Result<String> {
		app_files.read_string_asset(BANNER_ASSET_PATH.join("banner.txt"))
	}
}

impl UiElement for HomeScreen<'_> {
	type State = ScreenHandleData;

	fn event(
		&mut self,
		handle: &mut Self::State,
		event: &Event,
	) -> crate::Result<()> {
		if let Event::Input(InputEvent::Key(key)) = event {
			if key.code == KeyCode::Char('q') {
				handle.event_sender.send(ScreenEvent::Close.into())?;
			}
		}
		self.banner.event(&mut self.banner_state, event)?;
		Ok(())
	}

	fn tick(&mut self, _state: &mut Self::State) -> crate::Result<()> {
		self.banner.tick(&mut self.banner_state)?;
		Ok(())
	}

	fn render(&self, _state: &Self::State, buffer: &mut Buffer, area: Rect) {
		self.banner.render(&self.banner_state, buffer, area);
	}
}

impl Screen for HomeScreen<'_> {
	fn get_init_state<'a>(
		&self,
		builder: &'a mut ScreenMetadataBuilder,
	) -> &'a mut ScreenMetadataBuilder {
		builder.title("Terminal Arcade 🕹️")
	}
}
