#![recursion_limit = "512"]
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

    fn set_app_lite_dir(&self, dir: &str) {
        let dir = CString::new(dir).unwrap().into_raw();
        unsafe {
            cpp!([self as "agora::recording::RecordingConfig*", dir as "const char *"] {
                self->appliteDir = dir;
            })
        }
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

    fn set_canvas_width(&self, width: u32) {
        unsafe {
            cpp!([  self as "agora::linuxsdk::VideoMixingLayout*",
                    width as "int"] {
                self->canvasWidth = width;
            })
        }   
    }

    fn canvas_width(&self) -> u32 {
        unsafe {
            cpp!([self as "agora::linuxsdk::VideoMixingLayout*"] -> u32 as "int" {
                return self->canvasWidth;
            })
        }
    }
    
    fn set_canvas_height(&self, height: u32) {
        unsafe {
            cpp!([  self as "agora::linuxsdk::VideoMixingLayout*",
                    height as "int"] {
                self->canvasHeight = height;
            })
        }   
    }

    fn canvas_height(&self) -> u32 {
        unsafe {
            cpp!([self as "agora::linuxsdk::VideoMixingLayout*"] -> u32 as "int" {
                return self->canvasHeight;
            })
        }
    }

    fn set_background_rgb(&self, rgb: &str) {
        let rgb = CString::new(rgb).unwrap().into_raw();
        unsafe {
            cpp!([  self as "agora::linuxsdk::VideoMixingLayout*",
                    rgb as "const char *"] {
                self->backgroundColor = rgb;
            })
        }
    }

    fn background_rgb(&self) -> Result<&str, std::str::Utf8Error> {
        let p = unsafe {
            cpp!([self as "agora::linuxsdk::VideoMixingLayout*"] -> *const c_char as "const char *" {
                return self->backgroundColor;
            })
        } as *const i8;
        let c = unsafe {CStr::from_ptr(p)};
        c.to_str()
    }

    fn set_region_count(&self, count: u32) {
        unsafe {
            cpp!([  self as "agora::linuxsdk::VideoMixingLayout*",
                    count as "int"] {
                self->regionCount = count;
            })
        }  
    }

    fn set_region(&self, index: u32, x: u32, y: u32, width: u32, height: u32, uid: u32) {
        unsafe {
            cpp!([  self as "agora::linuxsdk::VideoMixingLayout*",
                    index as "int",
                    x as "int",
                    y as "int",
                    width as "int",
                    height as "int",
                    uid as "int"] {
                
                agora::linuxsdk::VideoMixingLayout::Region * regionList = new agora::linuxsdk::VideoMixingLayout::Region[1];
                regionList[0].uid = uid;
                regionList[0].x = x;
                regionList[0].width = width;
                regionList[0].height = height;
                regionList[0].alpha = 1;
                regionList[0].renderMode = 0;
                self->regions = regionList;
            })
        }  
    }
}

// cpp!{{
//     class AgoraSdk : public agora::AgoraSdk {
//     protected:
//         virtual void onError(int error, agora::linuxsdk::STAT_CODE_TYPE stat_code) override {
//             rust!(OnErrorImpl [this : AgoraSdk as "AgoraSdk*", error : u32 as "int", stat_code : u32 as "int"] {
//                 this.on_error(error, stat_code);
//             });
//         }

//         virtual void onUserJoined(agora::linuxsdk::uid_t uid, agora::linuxsdk::UserJoinInfos &infos) override {
//             rust!(OnUserImpl [this : AgoraSdk as "AgoraSdk*", uid : u32 as "int"] {
//                 this.on_user_joined(uid);
//             });
//         }
//     };
// }}

pub trait CallbackTrait {
    fn on_error(&mut self, error: u32, stat_code: u32);
    fn on_user_joined(&mut self, uid: u32);
}

cpp!{{ 
    struct CallbackPtr { void *a,*b; };
    class AgoraSdkEvents :  virtual public agora::recording::IRecordingEngineEventHandler {
        public:
        CallbackPtr callback;
        protected:
        virtual void onError(int error, agora::linuxsdk::STAT_CODE_TYPE stat_code) {
            //sdk->stoppedOnError();
            rust!(OnErrorImpl [callback : &mut dyn CallbackTrait as "CallbackPtr", error: u32 as "int", stat_code : u32 as "int"] {
                callback.on_error(error, stat_code)
            });
        }
        
        virtual void onWarning(int warn) {
        }
        virtual void onJoinChannelSuccess(const char * channelId, agora::linuxsdk::uid_t uid) {
        }
        virtual void onLeaveChannel(agora::linuxsdk::LEAVE_PATH_CODE code) {
        }
    
        virtual void onUserJoined(agora::linuxsdk::uid_t uid, agora::linuxsdk::UserJoinInfos &infos) {
            rust!(OnUserJoinedImpl [callback : &mut dyn CallbackTrait as "CallbackPtr", uid: u32 as "int"] {
                callback.on_user_joined(uid)
            });
        }
    
        virtual void onRemoteVideoStreamStateChanged(agora::linuxsdk::uid_t uid, agora::linuxsdk::RemoteStreamState state, agora::linuxsdk::RemoteStreamStateChangedReason reason) {
        }
    
        virtual void onRemoteAudioStreamStateChanged(agora::linuxsdk::uid_t uid, agora::linuxsdk::RemoteStreamState state, agora::linuxsdk::RemoteStreamStateChangedReason reason) {
        }
    
        virtual void onUserOffline(agora::linuxsdk::uid_t uid, agora::linuxsdk::USER_OFFLINE_REASON_TYPE reason) {
        }
    
        virtual void audioFrameReceived(unsigned int uid, const agora::linuxsdk::AudioFrame *frame) const {
        }
        virtual void videoFrameReceived(unsigned int uid, const agora::linuxsdk::VideoFrame *frame) const {
        }
        virtual void onActiveSpeaker(uid_t uid) {
        }
        virtual void onAudioVolumeIndication(const agora::linuxsdk::AudioVolumeInfo* speakers, unsigned int speakerNum) {
        }
    
        virtual void onFirstRemoteVideoDecoded(uid_t uid, int width, int height, int elapsed) {
        }
    
        virtual void onFirstRemoteAudioFrame(uid_t uid, int elapsed) {}
    
        virtual void onReceivingStreamStatusChanged(bool receivingAudio, bool receivingVideo) {}
    
        virtual void onConnectionLost() {}
    
        virtual void onConnectionInterrupted() {}
    
        virtual void onRejoinChannelSuccess(const char* channelId, uid_t uid) {}
    
        virtual void onConnectionStateChanged(agora::linuxsdk::ConnectionStateType state, agora::linuxsdk::ConnectionChangedReasonType reason){}
    
        virtual void onRecordingStats(const agora::linuxsdk::RecordingStats& stats){}
    
        virtual void onRemoteVideoStats(uid_t uid, const agora::linuxsdk::RemoteVideoStats& stats){}
    
        virtual void onRemoteAudioStats(uid_t uid, const agora::linuxsdk::RemoteAudioStats& stats){}
            
        virtual void onLocalUserRegistered(uid_t uid, const char* userAccount){}
    
        virtual void onUserInfoUpdated(uid_t uid, const agora::linuxsdk::UserInfo& info){}
    };    
}}

pub struct AgoraSdkEvents {
    pub rawptr: *mut u32,
    initialised: bool,
    on_error: Option<Box<dyn FnMut(u32, u32)>>,
    on_user_joined: Option<Box<dyn FnMut(u32)>>,    
}

impl CallbackTrait for AgoraSdkEvents {
    
    fn on_error(&mut self, error: u32, stat_code: u32) {
        self.on_error.as_mut().unwrap()(error, stat_code);
    }

    fn on_user_joined(&mut self, uid: u32) {
        self.on_user_joined.as_mut().unwrap()(uid);
    }
}

impl AgoraSdkEvents {
    pub fn new() -> Self {
        let rawptr = unsafe {
            cpp!([] -> *mut u32  as "agora::recording::IRecordingEngineEventHandler*" {
                return new AgoraSdkEvents();
            })
        };

        AgoraSdkEvents {
            rawptr,
            initialised: false,
            on_error: None,
            on_user_joined: None,
        }
    }

    pub fn set_on_error(&mut self, on_error: impl FnMut(u32, u32) + 'static) {
        self.on_error = Some(Box::new(on_error));
        self.connect()
    }

    pub fn set_on_user_joined(&mut self, on_user_joined: impl FnMut(u32) + 'static) {
        self.on_user_joined = Some(Box::new(on_user_joined));
        self.connect()
    }

    pub fn connect(&mut self) {
        if self.initialised {
            return;
        }
        
        self.initialised = true;
        
        let inst_ptr: &dyn CallbackTrait = self as &dyn CallbackTrait;    
        let rawptr = self.rawptr;
        unsafe {
            cpp!([  rawptr as "AgoraSdkEvents*",
                    inst_ptr as "CallbackPtr"] {
                rawptr->callback = inst_ptr;
            })
        } 
    }
}

#[derive(Debug)]
pub struct AgoraSdk {
    sdk: *mut u32,
}

impl AgoraSdk {
    pub fn new() -> Self {
        let sdk = unsafe {
            cpp!([] -> *mut u32  as "agora::AgoraSdk*" {
                return new agora::AgoraSdk();
            })
        };

        AgoraSdk {
            sdk,
        }
    }

    pub fn set_handler(&mut self, events: &AgoraSdkEvents) {
        unsafe {
            let handler = events.rawptr;
            let me = self.sdk;
            cpp!([me as "agora::AgoraSdk*", handler as "agora::recording::IRecordingEngineEventHandler*"] {
                me->setHandler(handler);
            })
        };
    }

    pub fn set_keep_last_frame(&self, keep : bool) {
        let me = self.sdk;
        unsafe {
            cpp!([me as "agora::AgoraSdk*", keep as "bool"] {
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
            cpp!([  me as "agora::AgoraSdk*", 
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
            cpp!([me as "agora::AgoraSdk*",
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
            cpp!([me as "agora::AgoraSdk*"] -> bool as "bool" {
                    return me->leaveChannel();
                }
            )
        }
    }

    pub fn set_video_mixing_layout(&self, layout: &Layout) -> u32 {
        let me = self.sdk;
        unsafe {
            cpp!([  me as "agora::AgoraSdk*", 
                    layout as "agora::linuxsdk::VideoMixingLayout*"] -> u32 as "int" {
                
                return me->setVideoMixingLayout(*layout);
            })
        }        
    }

    pub fn release(&self) -> bool {
        let me = self.sdk;
        unsafe {
            cpp!([me as "agora::AgoraSdk*"] -> bool as "bool" {
                return me->release();
            })
        }
    }
}

impl Drop for AgoraSdk {
    fn drop(&mut self) {
        self.leave_channel();
        let me = self.sdk;
        unsafe {
            cpp!([me as "agora::AgoraSdk*"] {
                delete me;
            })
        };
    }    
}

impl Clone for AgoraSdk {
    fn clone(&self) -> Self {
        
        AgoraSdk {
            sdk: self.sdk,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};
    use std::sync::Arc;

    // https://github.com/andyjsbell/agora-record/blob/master/build-node-gyp/src/agora_node_ext/agora_node_recording.cpp
    #[test]
    fn recorder_create() {
        
        let mut events = AgoraSdkEvents::new();
        let mut sdk = AgoraSdk::new();
        sdk.set_handler(&events);

        events.set_on_error(|error, stat_code| {
            println!("on_error -> {} {}", error, stat_code);
        });

        let config = Config::new();
           
        config.set_app_lite_dir("/home/andy/devel");
        config.set_mixing_enabled(true);
        config.set_mixed_video_audio(MixedAvCodecType::MixedAvCodecV2);
        config.set_idle_limit_sec(300);        
        config.set_channel_profile(ChannelProfile::LiveBroadcast);
        config.set_trigger_mode(TriggerMode::Automatic);
        config.set_mix_resolution(640, 480, 15, 500);        
        config.set_audio_indication_interval(0);
        
        let channel = "demo";
        
        {
            let sdk = sdk.clone();
            let on_user = move |uid| {
                println!("on_user_joined -> {}", uid);   
                let layout = Layout::new();
                layout.set_region_count(1);
                layout.set_region(0, 0, 0, 1, 1, uid);
                layout.set_background_rgb("#00ff00");

                sdk.set_video_mixing_layout(&layout);
            };
            events.set_on_user_joined(on_user);
        }

        sdk.create_channel("e544083a6e54401c8f729815b2a42022", "", channel, 0, &config);
        
        thread::sleep(time::Duration::from_millis(5000));
    }

    #[test]
    fn recorder_release() {
        let sdk = AgoraSdk::new();
        assert!(sdk.release());
    }

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