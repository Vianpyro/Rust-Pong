use ggez::audio::SoundSource;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/sfx"]
pub struct Asset;

pub fn play_embedded_sound(ctx: &mut ggez::Context, name: &str) -> ggez::GameResult<()> {
    if let Some(data) = Asset::get(name) {
        let bytes = data.data.as_ref();
        let sound_data = ggez::audio::SoundData::from_bytes(bytes);
        let mut src = ggez::audio::Source::from_data(ctx, sound_data)?;
        src.play_detached(ctx)?;
        Ok(())
    } else {
        Err(ggez::GameError::ResourceLoadError(format!("Embedded SFX not found: {}", name)))
    }
}
