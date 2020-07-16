use cpp::cpp;
use cpp::cpp_class;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;

cpp!{{
    #include <iostream>
    #include "src/cpp/agorasdk/AgoraSdk.h"
    using std::string;
}}

#[derive(PartialEq, PartialOrd, Debug)]
enum TriggerMode {
    Automatic = 0,
    Manual = 1,
    Unknown = 2
}

impl TriggerMode {
    fn value(&self) -> u32 {
        match *self {
            TriggerMode::Automatic => 0,
            TriggerMode::Manual => 1,
            TriggerMode::Unknown => 2
        }
    }
}

impl From<u32> for TriggerMode {
    fn from(orig: u32) -> Self {
        match orig {
            0 => return TriggerMode::Automatic,
            1 => return TriggerMode::Manual,
            _ => return TriggerMode::Unknown
        };
    }
}

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

#[derive(PartialEq, PartialOrd, Debug)]
enum ChannelProfile
{
    Communication = 0,
    LiveBroadcast = 1,
    Unknown = 2
}

impl ChannelProfile {
    fn value(&self) -> u32 {
        match *self {
            ChannelProfile::Communication => 0,
            ChannelProfile::LiveBroadcast => 1,
            ChannelProfile::Unknown => 2
        }
    }
}

impl From<u32> for ChannelProfile {
    fn from(orig: u32) -> Self {
        match orig {
            0 => return ChannelProfile::Communication,
            1 => return ChannelProfile::LiveBroadcast,
            _ => return ChannelProfile::Unknown
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
                
    fn set_channel_profile(&self, profile: ChannelProfile) {
        let profile = profile.value();
        unsafe {
            cpp!([  self as "agora::recording::RecordingConfig*",
                    profile as "agora::linuxsdk::CHANNEL_PROFILE_TYPE"] {
                self->channelProfile = profile;
            })
        }
    }

    fn channel_profile(&self) -> ChannelProfile {
        unsafe {
            cpp!([self as "agora::recording::RecordingConfig*"] -> u32 as "agora::linuxsdk::CHANNEL_PROFILE_TYPE" {
                return self->channelProfile;
            })
        }.into()
    }

    fn set_trigger_mode(&self, trigger: TriggerMode) {
        let trigger = trigger.value();
        unsafe {
            cpp!([  self as "agora::recording::RecordingConfig*",
                    trigger as "agora::linuxsdk::TRIGGER_MODE_TYPE"] {
                self->triggerMode = trigger;
            })
        }
    }

    fn trigger_mode(&self) -> TriggerMode {
        unsafe {
            cpp!([self as "agora::recording::RecordingConfig*"] -> u32 as "agora::linuxsdk::TRIGGER_MODE_TYPE" {
                return self->triggerMode;
            })
        }.into()
    }

    fn set_mix_resolution(&self, width: u32, height: u32, fps: u32, kbps: u32) {
        let mix_resolution = format!("{},{},{},{}", width, height, fps, kbps);
        
        let mix_resolution = CString::new(mix_resolution).unwrap().into_raw();
        unsafe {
            cpp!([  self as "agora::recording::RecordingConfig*",
                    mix_resolution as "const char *"] {
                self->mixResolution = mix_resolution;
            })
        }
    }

    fn mix_resolution(&self) -> (u32, u32, u32, u32) {
        let p = unsafe {
            cpp!([self as "agora::recording::RecordingConfig*"] -> *const c_char as "const char *" {
                return self->mixResolution;
            })
        } as *const i8;
        let c = unsafe {CStr::from_ptr(p)};
        let mix = c.to_str().unwrap();
        let split = mix.split(",").collect::<Vec<_>>();
        let vec: Vec<u32> = split.iter().map(|s| s.parse::<u32>().unwrap()).collect();
        (vec[0], vec[1], vec[2], vec[3])
    }

    fn set_audio_indication_interval(&self, interval: u32) {
        unsafe {
            cpp!([  self as "agora::recording::RecordingConfig*",
                    interval as "int"] {
                self->audioIndicationInterval = interval;
            })
        }   
    }

    fn audio_indication_interval(&self) -> u32 {
        unsafe {
            cpp!([self as "agora::recording::RecordingConfig*"] -> u32 as "int" {
                return self->audioIndicationInterval;
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

cpp!{{
    class AgoraSdk : public agora::AgoraSdk {
    protected:
       virtual void onError(int error, agora::linuxsdk::STAT_CODE_TYPE stat_code) override {
            rust!(My_Rust_MethodImpl [this : AgoraSdk as "AgoraSdk*", error : u32 as "int", stat_code : u32 as "int"] {
                this.on_error(error, stat_code);
            });
       }
    };
}}

pub struct AgoraSdk {
    sdk: *mut u32,
}

impl AgoraSdk {
    pub fn new() -> AgoraSdk {
        let sdk = unsafe {
            cpp!([] -> *mut u32  as "AgoraSdk*" {
                return new AgoraSdk();
            })
        };

        AgoraSdk {
            sdk
        }
    }

    pub fn set_keep_last_frame(&self, keep : bool) {
        let me = self.sdk;
        unsafe {
            cpp!([me as "AgoraSdk*", keep as "bool"] {
                    return me->setKeepLastFrame(keep);
                }
            )
        }
    }

    pub fn create_channel(&self, app_id: &str, channel_key: &str, name: &str, uid: u32, config: &Config) -> bool {
        
        let me = self.sdk;
        let app_id = CString::new(app_id).unwrap().into_raw();
        let name = CString::new(name).unwrap().into_raw();
        let channel_key = CString::new(channel_key).unwrap().into_raw();
        
        unsafe {
            cpp!([  me as "AgoraSdk*", 
                    app_id as "const char *",
                    channel_key as "const char *",
                    name as "const char *",
                    uid as "int",
                    config as "agora::recording::RecordingConfig*"
                    ] -> bool as "bool" {
                        return me->createChannel(app_id, channel_key, name, uid, *config);
                    }

            )
        }
    }
    
    pub fn update_mix_mode_setting(&self, width: u32, height: u32, is_video_mix: bool) {
        let me = self.sdk;
        unsafe {
            cpp!([me as "AgoraSdk*",
                width as "int",
                height as "int", 
                is_video_mix as "bool"] {
                    me->updateMixModeSetting(width, height, is_video_mix);
                }

            )
        }
    }

    pub fn leave_channel(&self) -> bool {
        let me = self.sdk;
        unsafe {
            cpp!([me as "AgoraSdk*"] -> bool as "bool" {
                    return me->leaveChannel();
                }
            )
        }
    }

    pub fn set_video_mixing_layout(&self, layout: &Layout) -> u32 {
        let me = self.sdk;
        unsafe {
            cpp!([  me as "AgoraSdk*", 
                    layout as "agora::linuxsdk::VideoMixingLayout*"] -> u32 as "int" {
                
                return me->setVideoMixingLayout(*layout);
            })
        }        
    }

    pub fn release(&self) -> bool {
        let me = self.sdk;
        unsafe {
            cpp!([me as "AgoraSdk*"] -> bool as "bool" {
                return me->release();
            })
        }
    }

    fn on_error(&self, error: u32, stat_code: u32) {
        println!("event - {} {}", error, stat_code)
    }
}

impl Drop for AgoraSdk {
    fn drop(&mut self) {
        let me = self.sdk;
        unsafe {
            cpp!([me as "AgoraSdk*"] {
                delete me;
            })
        };
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recorder_keep_last_frame() {
        let sdk = AgoraSdk::new();
        sdk.set_keep_last_frame(true);
    }

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
    
    #[test]
    fn config_set_channel_profile() {
        let config = Config::new();
        config.set_channel_profile(ChannelProfile::LiveBroadcast);
        assert!(config.channel_profile() == ChannelProfile::LiveBroadcast);
    }
    
    #[test]
    fn config_set_trigger_mode() {
        let config = Config::new();
        config.set_trigger_mode(TriggerMode::Automatic);
        assert!(config.trigger_mode() == TriggerMode::Automatic);
    }

    #[test]
    fn config_set_mixed_resolution() {
        let config = Config::new();
        config.set_mix_resolution(1920, 1080, 30, 2000);
        assert!(config.mix_resolution().0 == 1920);
        assert!(config.mix_resolution().1 == 1080);
        assert!(config.mix_resolution().2 == 30);
        assert!(config.mix_resolution().3 == 2000);
    }

    #[test]
    fn config_set_audio_indication_interval() {
        let config = Config::new();
        config.set_audio_indication_interval(10);
        assert!(config.audio_indication_interval() == 10);
    }
}