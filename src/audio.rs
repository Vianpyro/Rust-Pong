use ggez::audio::{SoundSource, Source};
use ggez::{Context, GameResult};
use std::collections::HashMap;

pub struct SfxManager {
    sounds: HashMap<String, Source>,
}

impl SfxManager {
    pub fn new() -> Self {
        Self { sounds: HashMap::new() }
    }

    pub fn load(&mut self, ctx: &mut Context, name: &str, path: &str) -> GameResult<()> {
        let src = Source::new(ctx, path)?;
        self.sounds.insert(name.to_string(), src);
        Ok(())
    }

    pub fn play(&mut self, name: &str, ctx: &mut Context) -> GameResult<()> {
        if let Some(s) = self.sounds.get_mut(name) {
            s.play_detached(ctx)?;
            Ok(())
        } else {
            Err(ggez::GameError::ResourceLoadError(format!("SFX not found: {}", name)))
        }
    }
}
