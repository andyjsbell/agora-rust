use cpp::cpp;
use cpp::cpp_class;

cpp!{{
    #include "src/cpp/agorasdk/AgoraSdk.h"
    using std::string;
}}

cpp_class!(pub unsafe struct Config as "agora::recording::RecordingConfig");
impl Config {
    fn new() -> Self {
        unsafe { cpp!([] -> Config as "agora::recording::RecordingConfig" {return agora::recording::RecordingConfig();}) }
    }
}

cpp_class!(pub unsafe struct Recorder as "agora::AgoraSdk");
impl Recorder {
    fn new() -> Self {
        unsafe { cpp!([] -> Recorder as "agora::AgoraSdk" {return agora::AgoraSdk();}) }
    }

    fn create_channel(&self, app_id: &str, channel_key: &str, name: &str, uid: u32, config: &Config) -> bool {
        
        let app_id = app_id.as_ptr();
        let name = name.as_ptr();
        let channel_key = channel_key.as_ptr();
        
        unsafe {
            cpp!([  self as "agora::AgoraSdk*", 
                    app_id as "const char *",
                    channel_key as "const char *",
                    name as "const char *",
                    uid as "int",
                    config as "agora::recording::RecordingConfig*"
                    ] -> bool as "bool" {
                        return self->createChannel(app_id, channel_key, name, uid, *config);
                    }

            )
        }
    }
    
    fn update_mix_mode_setting(&self, width: u32, height: u32, is_video_mix: bool) {
        unsafe {
            cpp!([self as "agora::AgoraSdk*",
                width as "int",
                height as "int", 
                is_video_mix as "bool"] {
                    self->updateMixModeSetting(width, height, is_video_mix);
                }

            )
        }
    }

    fn leave_channel(&self) -> bool {
        unsafe {
            cpp!([self as "agora::AgoraSdk*"] -> bool as "bool" {
                    return self->leaveChannel();
                }
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let recorder = Recorder::new();

    }
}