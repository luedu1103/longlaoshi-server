use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::audio::{self, AudioSpeechRequest, TTS_1};

pub fn generate_speech(string: &String, api_token: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(api_token);

    let req = AudioSpeechRequest::new(
        TTS_1.to_string(),
        string.to_string(),
        audio::VOICE_SHIMMER.to_string(),
        String::from("Voz1.mp3"),
    );

    let result = client.audio_speech(req)?;
    println!("{:?}", result);

    Ok(())
}