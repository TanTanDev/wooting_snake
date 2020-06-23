use rand::Rng;
use rodio;
use rodio::Device;
use rodio::Source;
use std::collections::HashMap;
use std::fs::File;

use std::convert::AsRef;
use std::io;
use std::io::prelude::*;
use std::sync::Arc;

type Sounds = Vec<Sound>;
type SoundsMap = HashMap<SoundType, Sounds>;

#[derive(Eq, PartialEq, std::hash::Hash, Clone, Copy)]
pub enum SoundType {
    Click,
    Death,
    Eat,
    Step,
    Warp,
}

pub struct SoundManager {
    sounds_map: SoundsMap,
    device: Option<Device>,
    rng: rand::rngs::ThreadRng,
}
pub struct Sound(Arc<Vec<u8>>);
impl AsRef<[u8]> for Sound {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Sound {
    pub fn load(filename: &str) -> io::Result<Sound> {
        let mut buf = Vec::new();
        let mut file = File::open(filename)?;
        file.read_to_end(&mut buf)?;
        Ok(Sound(Arc::new(buf)))
    }
    pub fn cursor(self: &Self) -> io::Cursor<Sound> {
        io::Cursor::new(Sound(self.0.clone()))
    }
    pub fn decoder(self: &Self) -> rodio::Decoder<io::Cursor<Sound>> {
        rodio::Decoder::new(self.cursor()).unwrap()
    }
}

fn create_source(sounds_map: &mut SoundsMap, sound_type: SoundType, path: &str) {
    let sounds = match sounds_map.get_mut(&sound_type) {
        Some(decoders) => decoders,
        None => {
            sounds_map.insert(sound_type, Sounds::new());
            sounds_map.get_mut(&sound_type).unwrap()
        }
    };
    let sound = Sound::load(path).unwrap();
    sounds.push(sound);
}

impl SoundManager {
    pub fn new() -> Self {
        let maybe_device = rodio::default_output_device();
        let mut sounds_map = HashMap::new();
        create_source(&mut sounds_map, SoundType::Click, "resources/click.wav");
        create_source(&mut sounds_map, SoundType::Death, "resources/death.wav");
        create_source(&mut sounds_map, SoundType::Eat, "resources/eat1.wav");
        create_source(&mut sounds_map, SoundType::Eat, "resources/eat2.wav");
        create_source(&mut sounds_map, SoundType::Eat, "resources/eat3.wav");
        create_source(&mut sounds_map, SoundType::Step, "resources/step1.wav");
        create_source(&mut sounds_map, SoundType::Step, "resources/step2.wav");
        create_source(&mut sounds_map, SoundType::Warp, "resources/warp.wav");
        SoundManager {
            device: maybe_device,
            sounds_map,
            rng: rand::thread_rng(),
        }
    }

    pub fn play(&mut self, sound_type: SoundType) {
        let maybe_sounds = self.sounds_map.get(&sound_type);
        if let Some(sounds) = maybe_sounds {
            let random_index = self.rng.gen_range(0, sounds.len());
            let maybe_sound = sounds.get(random_index);
            if let Some(sound) = &maybe_sound {
                if let Some(device) = &self.device {
                    rodio::play_raw(device, sound.decoder().convert_samples());
                }
            }
        }
    }
}
