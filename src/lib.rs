use cpp::cpp;
use cpp::cpp_class;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;

/*
                config.idleLimitSec = 10;
                // config.decodeVideo = agora::linuxsdk::VIDEO_FORMAT_MIX_JPG_FILE_TYPE;
                config.channelProfile = agora::linuxsdk::CHANNEL_PROFILE_LIVE_BROADCASTING;
                // config.captureInterval = 1;
                config.triggerMode = agora::linuxsdk::AUTOMATICALLY_MODE;
                config.mixResolution = "640,480,15,500";

                                config.mixResolution = &mixResolution[0u];
                config.audioIndicationInterval = 0;
*/
cpp!{{
    #include "src/cpp/agorasdk/AgoraSdk.h"
    using std::string;
}}

#[derive(PartialEq, PartialOrd, Debug)]
enum MixedAvCodecType {
    MixedAvDefault = 0,  
    MixedAvCodecV1 = 1,
    MixedAvCodecV2 = 2,
    Unknown = 3,
}

impl MixedAvCodecType {
    fn value(&self) -> u32 {
        match *self {
            MixedAvCodecType::MixedAvDefault => 0,
            MixedAvCodecType::MixedAvCodecV1 => 1,
            MixedAvCodecType::MixedAvCodecV2 => 2,
            MixedAvCodecType::Unknown => 3
        }
    }
}

impl From<u32> for MixedAvCodecType {
    fn from(orig: u32) -> Self {
        match orig {
            0 => return MixedAvCodecType::MixedAvDefault,
            1 => return MixedAvCodecType::MixedAvCodecV1,
            2 => return MixedAvCodecType::MixedAvCodecV2,
            _ => return MixedAvCodecType::Unknown
        };
    }
}

cpp_class!(pub unsafe struct Config as "agora::recording::RecordingConfig");
impl Config {
    fn new() -> Self {
        unsafe { cpp!([] -> Config as "agora::recording::RecordingConfig" {return agora::recording::RecordingConfig();}) }
    }

    fn is_mixing_enabled(&self) -> bool {
        unsafe {
            cpp!([self as "agora::recording::RecordingConfig*"] -> bool as "bool" {
                return self->isMixingEnabled;
            })
        }    
    }

    fn set_mixing_enabled(&self, enabled: bool) {
        unsafe {
            cpp!([  self as "agora::recording::RecordingConfig*",
                    enabled as "bool"] {
                self->isMixingEnabled = enabled;
            })
        }    
    }

    fn set_recording_path(&self, path: &str) {
        let path = CString::new(path).unwrap().into_raw();
        unsafe {
            cpp!([  self as "agora::recording::RecordingConfig*",
                    path as "const char *"] {
                self->appliteDir = path;
            })
        }
    }

    fn recording_path(&self) -> Result<&str, std::str::Utf8Error> {
        let p = unsafe {
            cpp!([self as "agora::recording::RecordingConfig*"] -> *const c_char as "const char *" {
                return self->appliteDir;
            })
        } as *const i8;
        let c = unsafe {CStr::from_ptr(p)};
        c.to_str()
    }

    fn set_config_path(&self, path: &str) {
        let path = CString::new(path).unwrap().into_raw();
        unsafe {
            cpp!([  self as "agora::recording::RecordingConfig*",
                    path as "const char *"] {
                self->cfgFilePath = path;
            })
        }
    }

    fn config_path(&self) -> Result<&str, std::str::Utf8Error> {
        let p = unsafe {
            cpp!([self as "agora::recording::RecordingConfig*"] -> *const c_char as "const char *" {
                return self->cfgFilePath;
            })
        } as *const i8;
        let c = unsafe {CStr::from_ptr(p)};
        c.to_str()
    }

    fn set_mixed_video_audio(&self, mixed_type: MixedAvCodecType) {
        let mixed_type = mixed_type.value();
        unsafe {
            cpp!([  self as "agora::recording::RecordingConfig*",
                    mixed_type as "agora::linuxsdk::MIXED_AV_CODEC_TYPE"] {
                self->mixedVideoAudio = mixed_type;
            })
        }
    }

    fn mixed_video_audio(&self) -> MixedAvCodecType {
        unsafe {
            cpp!([self as "agora::recording::RecordingConfig*"] -> u32 as "agora::linuxsdk::MIXED_AV_CODEC_TYPE" {
                return self->mixedVideoAudio;
            })
        }.into()
    }
                
    fn set_idle_limit_sec(&self, limit: u32) {
        unsafe {
            cpp!([  self as "agora::recording::RecordingConfig*",
                    limit as "int"] {
                self->idleLimitSec = limit;
            })
        }   
    }

    fn idle_limit_sec(&self) -> u32{
        unsafe {
            cpp!([self as "agora::recording::RecordingConfig*"] -> u32 as "int" {
                return self->idleLimitSec;
            })
        }
    }
}

cpp_class!(pub unsafe struct Layout as "agora::linuxsdk::VideoMixingLayout");
impl Layout {
    fn new() -> Self {
        unsafe { cpp!([] -> Layout as "agora::linuxsdk::VideoMixingLayout" {return agora::linuxsdk::VideoMixingLayout();}) }
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

    fn set_video_mixing_layout(&self, layout: &Layout) -> u32 {
        unsafe {
            cpp!([  self as "agora::AgoraSdk*", 
                    layout as "agora::linuxsdk::VideoMixingLayout*"] -> u32 as "int" {
                
                return self->setVideoMixingLayout(*layout);
            })
        }        
    }

    fn release(&self) -> bool {
        unsafe {
            cpp!([self as "agora::AgoraSdk*"] -> bool as "bool" {
                return self->release();
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_mixing_enabled() {
        let config = Config::new();
        assert!(!config.is_mixing_enabled(), "should be false from the start");
        config.set_mixing_enabled(true);
        assert!(config.is_mixing_enabled(), "should be true after updating");
    }

    #[test]
    fn config_set_recording_path() {
        let config = Config::new();
        let str = "test";
        config.set_recording_path(str);
        assert!(config.recording_path().is_ok());
        let path = config.recording_path().unwrap();

        assert!(path == str);
    }
    
    #[test]
    fn config_set_config_path() {
        let config = Config::new();
        let str = "test";
        config.set_config_path(str);
        assert!(config.config_path().is_ok());
        let path = config.config_path().unwrap();

        assert!(path == str);
    }

    #[test]
    fn config_set_mixed_video_audio() {
        let config = Config::new();
        config.set_mixed_video_audio(MixedAvCodecType::MixedAvCodecV2);
        assert!(config.mixed_video_audio() == MixedAvCodecType::MixedAvCodecV2);
    }

    #[test]
    fn config_set_idel_limit_sec() {
        let config = Config::new();
        config.set_idle_limit_sec(10);
        assert!(config.idle_limit_sec() == 10);
    }
}