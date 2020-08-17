#![recursion_limit = "1024"]
use cpp::cpp;
use cpp::cpp_class;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::env;

cpp!{{
    #include <iostream>
    #include "src/cpp/agorasdk/AgoraSdk.h"
    using std::string;
}}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum TriggerMode {
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
pub enum MixedAvCodecType {
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
pub enum ChannelProfile
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
    pub fn new() -> Self {
        unsafe { cpp!([] -> Config as "agora::recording::RecordingConfig" {return agora::recording::RecordingConfig();}) }
    }

    pub fn set_app_lite_dir(&self, dir: &str) {
        let dir = CString::new(dir).unwrap().into_raw();
        unsafe {
            cpp!([self as "agora::recording::RecordingConfig*", dir as "const char *"] {
                self->appliteDir = dir;
            })
        }
    }

    pub fn is_mixing_enabled(&self) -> bool {
        unsafe {
            cpp!([self as "agora::recording::RecordingConfig*"] -> bool as "bool" {
                return self->isMixingEnabled;
            })
        }    
    }

    pub fn set_mixing_enabled(&self, enabled: bool) {
        unsafe {
            cpp!([  self as "agora::recording::RecordingConfig*",
                    enabled as "bool"] {
                self->isMixingEnabled = enabled;
            })
        }    
    }

    pub fn set_recording_path(&self, path: &str) {
        let path = CString::new(path).unwrap().into_raw();
        unsafe {
            cpp!([  self as "agora::recording::RecordingConfig*",
                    path as "const char *"] {
                self->recordFileRootDir = path;
            })
        }
    }

    pub fn recording_path(&self) -> Result<&str, std::str::Utf8Error> {
        let p = unsafe {
            cpp!([self as "agora::recording::RecordingConfig*"] -> *const c_char as "const char *" {
                return self->recordFileRootDir;
            })
        } as *const i8;
        let c = unsafe {CStr::from_ptr(p)};
        c.to_str()
    }

    pub fn set_config_path(&self, path: &str) {
        let path = CString::new(path).unwrap().into_raw();
        unsafe {
            cpp!([  self as "agora::recording::RecordingConfig*",
                    path as "const char *"] {
                self->cfgFilePath = path;
            })
        }
    }

    pub fn config_path(&self) -> Result<&str, std::str::Utf8Error> {
        let p = unsafe {
            cpp!([self as "agora::recording::RecordingConfig*"] -> *const c_char as "const char *" {
                return self->cfgFilePath;
            })
        } as *const i8;
        let c = unsafe {CStr::from_ptr(p)};
        c.to_str()
    }

    pub fn set_mixed_video_audio(&self, mixed_type: MixedAvCodecType) {
        let mixed_type = mixed_type.value();
        unsafe {
            cpp!([  self as "agora::recording::RecordingConfig*",
                    mixed_type as "agora::linuxsdk::MIXED_AV_CODEC_TYPE"] {
                self->mixedVideoAudio = mixed_type;
            })
        }
    }

    pub fn mixed_video_audio(&self) -> MixedAvCodecType {
        unsafe {
            cpp!([self as "agora::recording::RecordingConfig*"] -> u32 as "agora::linuxsdk::MIXED_AV_CODEC_TYPE" {
                return self->mixedVideoAudio;
            })
        }.into()
    }
                
    pub fn set_idle_limit_sec(&self, limit: u32) {
        unsafe {
            cpp!([  self as "agora::recording::RecordingConfig*",
                    limit as "int"] {
                self->idleLimitSec = limit;
            })
        }   
    }

    pub fn idle_limit_sec(&self) -> u32{
        unsafe {
            cpp!([self as "agora::recording::RecordingConfig*"] -> u32 as "int" {
                return self->idleLimitSec;
            })
        }
    }
                
    pub fn set_channel_profile(&self, profile: ChannelProfile) {
        let profile = profile.value();
        unsafe {
            cpp!([  self as "agora::recording::RecordingConfig*",
                    profile as "agora::linuxsdk::CHANNEL_PROFILE_TYPE"] {
                self->channelProfile = profile;
            })
        }
    }

    pub fn channel_profile(&self) -> ChannelProfile {
        unsafe {
            cpp!([self as "agora::recording::RecordingConfig*"] -> u32 as "agora::linuxsdk::CHANNEL_PROFILE_TYPE" {
                return self->channelProfile;
            })
        }.into()
    }

    pub fn set_trigger_mode(&self, trigger: TriggerMode) {
        let trigger = trigger.value();
        unsafe {
            cpp!([  self as "agora::recording::RecordingConfig*",
                    trigger as "agora::linuxsdk::TRIGGER_MODE_TYPE"] {
                self->triggerMode = trigger;
            })
        }
    }

    pub fn trigger_mode(&self) -> TriggerMode {
        unsafe {
            cpp!([self as "agora::recording::RecordingConfig*"] -> u32 as "agora::linuxsdk::TRIGGER_MODE_TYPE" {
                return self->triggerMode;
            })
        }.into()
    }

    pub fn set_mix_resolution(&self, width: u32, height: u32, fps: u32, kbps: u32) {
        let mix_resolution = format!("{},{},{},{}", width, height, fps, kbps);
        
        let mix_resolution = CString::new(mix_resolution).unwrap().into_raw();
        unsafe {
            cpp!([  self as "agora::recording::RecordingConfig*",
                    mix_resolution as "const char *"] {
                self->mixResolution = mix_resolution;
            })
        }
    }

    pub fn mix_resolution(&self) -> (u32, u32, u32, u32) {
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

    pub fn set_audio_indication_interval(&self, interval: u32) {
        unsafe {
            cpp!([  self as "agora::recording::RecordingConfig*",
                    interval as "int"] {
                self->audioIndicationInterval = interval;
            })
        }   
    }

    pub fn audio_indication_interval(&self) -> u32 {
        unsafe {
            cpp!([self as "agora::recording::RecordingConfig*"] -> u32 as "int" {
                return self->audioIndicationInterval;
            })
        }
    }
}

cpp_class!(pub unsafe struct Region as "agora::linuxsdk::VideoMixingLayout::Region");
impl Region {
    pub fn new(uid: u32, x: f64, y: f64, width: f64, height: f64, alpha:f64, render_mode: u32) -> Self {
        unsafe { cpp!([ uid as "int", 
                        x as "double", 
                        y as "double", 
                        width as "double", 
                        height as "double",
                        alpha as "double", 
                        render_mode as "int"] -> Region as "agora::linuxsdk::VideoMixingLayout::Region" {
            
            agora::linuxsdk::VideoMixingLayout::Region region;
            region.uid = uid;
            region.x = x;
            region.y = y;
            region.width = width;
            region.height = height;
            region.alpha = alpha;
            region.renderMode = render_mode;
            return region;
        })}
    }

    pub fn uid(&self) -> u32 {
        unsafe {
            cpp!([self as "agora::linuxsdk::VideoMixingLayout::Region*"] -> u32 as "int" {
                return self->uid;
            })
        }
    }

    pub fn x(&self) -> f64 {
        unsafe {
            cpp!([self as "agora::linuxsdk::VideoMixingLayout::Region*"] -> f64 as "double" {
                return self->x;
            })
        }
    }

    pub fn y(&self) -> f64 {
        unsafe {
            cpp!([self as "agora::linuxsdk::VideoMixingLayout::Region*"] -> f64 as "double" {
                return self->y;
            })
        }
    }

    pub fn width(&self) -> f64 {
        unsafe {
            cpp!([self as "agora::linuxsdk::VideoMixingLayout::Region*"] -> f64 as "double" {
                return self->width;
            })
        }
    }

    pub fn height(&self) -> f64 {
        unsafe {
            cpp!([self as "agora::linuxsdk::VideoMixingLayout::Region*"] -> f64 as "double" {
                return self->height;
            })
        }
    }

    pub fn alpha(&self) -> f64 {
        unsafe {
            cpp!([self as "agora::linuxsdk::VideoMixingLayout::Region*"] -> f64 as "double" {
                return self->alpha;
            })
        }
    }

    pub fn render_mode(&self) -> u32 {
        unsafe {
            cpp!([self as "agora::linuxsdk::VideoMixingLayout::Region*"] -> u32 as "int" {
                return self->renderMode;
            })
        }
    }
}

cpp_class!(pub unsafe struct Layout as "agora::linuxsdk::VideoMixingLayout");
impl Layout {
    pub fn new() -> Self {
        unsafe { cpp!([] -> Layout as "agora::linuxsdk::VideoMixingLayout" {return agora::linuxsdk::VideoMixingLayout();}) }
    }

    pub fn set_canvas_width(&self, width: u32) {
        unsafe {
            cpp!([  self as "agora::linuxsdk::VideoMixingLayout*",
                    width as "int"] {
                self->canvasWidth = width;
            })
        }   
    }

    pub fn canvas_width(&self) -> u32 {
        unsafe {
            cpp!([self as "agora::linuxsdk::VideoMixingLayout*"] -> u32 as "int" {
                return self->canvasWidth;
            })
        }
    }
    
    pub fn set_canvas_height(&self, height: u32) {
        unsafe {
            cpp!([  self as "agora::linuxsdk::VideoMixingLayout*",
                    height as "int"] {
                self->canvasHeight = height;
            })
        }   
    }

    pub fn canvas_height(&self) -> u32 {
        unsafe {
            cpp!([self as "agora::linuxsdk::VideoMixingLayout*"] -> u32 as "int" {
                return self->canvasHeight;
            })
        }
    }

    pub fn set_background_rgb(&self, rgb: &str) {
        let rgb = CString::new(rgb).unwrap().into_raw();
        unsafe {
            cpp!([  self as "agora::linuxsdk::VideoMixingLayout*",
                    rgb as "const char *"] {
                self->backgroundColor = rgb;
            })
        }
    }

    pub fn background_rgb(&self) -> Result<&str, std::str::Utf8Error> {
        let p = unsafe {
            cpp!([self as "agora::linuxsdk::VideoMixingLayout*"] -> *const c_char as "const char *" {
                return self->backgroundColor;
            })
        } as *const i8;
        let c = unsafe {CStr::from_ptr(p)};
        c.to_str()
    }

    pub fn set_regions(&self, regions: Vec<Region>) {

        cpp! {{
            agora::linuxsdk::VideoMixingLayout::Region * regionList = nullptr;
        }}

        let count = regions.len() as u32;
        unsafe {
            cpp!([  self as "agora::linuxsdk::VideoMixingLayout*",
                    count as "int"] {
                
                self->regionCount = count;
                
                if (self->regions != nullptr)
                    delete self->regions;
                
                regionList = new agora::linuxsdk::VideoMixingLayout::Region[count];
            })
        }  

        let mut index = 0;
        for region in &regions {
            unsafe {
                cpp!([  self as "agora::linuxsdk::VideoMixingLayout*",
                        index as "int",
                        region as "agora::linuxsdk::VideoMixingLayout::Region*"] {
                    
                    regionList[index].uid = region->uid;
                    regionList[index].x = region->x;
                    regionList[index].y = region->y;
                    regionList[index].width = region->width;
                    regionList[index].height = region->height;
                    regionList[index].alpha = region->alpha;
                    regionList[index].renderMode = region->renderMode; 
                })
            }

            index = index + 1;
        }

        unsafe {
            cpp!([  self as "agora::linuxsdk::VideoMixingLayout*"] {
                self->regions = regionList;
            })
        }
    }

    pub fn get_regions(&self) -> Vec<Region> {
        let mut regions = Vec::new();
        let count = unsafe {
            cpp!([  self as "agora::linuxsdk::VideoMixingLayout*"] -> u32 as "int" {
                return self->regionCount;
            })
        };

        for index in 0..count {
            let region = unsafe {
                cpp!([  self as "agora::linuxsdk::VideoMixingLayout*", index as "int"] -> Region as "agora::linuxsdk::VideoMixingLayout::Region" {
                    return self->regions[index];
                })
            };
            regions.push(region);
        }
        regions
    }
}

pub trait CallbackTrait {
    fn on_error(&mut self, error: u32, stat_code: u32);
    fn on_user_joined(&mut self, uid: u32);
    fn on_user_left(&mut self, uid: u32); 
    fn on_channel_join_success(&mut self, channel: &str, uid: u32);   
}

cpp!{{ 
    struct CallbackPtr { void *a, *b; };
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
            (void)warn;
        }
        virtual void onJoinChannelSuccess(const char * channelId, agora::linuxsdk::uid_t uid) {
            rust!(OnJoinChannelSuccessImpl [callback : &mut dyn CallbackTrait as "CallbackPtr", channelId: *const i8 as "const char*", uid : u32 as "int"] {
                let channelId = unsafe {CStr::from_ptr(channelId)};
                callback.on_channel_join_success(channelId.to_str().unwrap_or(""), uid)
            });
        }
        virtual void onLeaveChannel(agora::linuxsdk::LEAVE_PATH_CODE code) {
            (void)code;
        }
    
        virtual void onUserJoined(agora::linuxsdk::uid_t uid, agora::linuxsdk::UserJoinInfos &infos) {
            (void)infos;
            rust!(OnUserJoinedImpl [callback : &mut dyn CallbackTrait as "CallbackPtr", uid: u32 as "int"] {
                callback.on_user_joined(uid)
            });
        }
    
        virtual void onRemoteVideoStreamStateChanged(agora::linuxsdk::uid_t uid, agora::linuxsdk::RemoteStreamState state, agora::linuxsdk::RemoteStreamStateChangedReason reason) {
            (void)uid;
            (void)state;
            (void)reason;
        }
    
        virtual void onRemoteAudioStreamStateChanged(agora::linuxsdk::uid_t uid, agora::linuxsdk::RemoteStreamState state, agora::linuxsdk::RemoteStreamStateChangedReason reason) {
            (void)uid;
            (void)state;
            (void)reason;
        }
    
        virtual void onUserOffline(agora::linuxsdk::uid_t uid, agora::linuxsdk::USER_OFFLINE_REASON_TYPE reason) {
            (void)reason;
            rust!(OnUserOfflineImpl [callback : &mut dyn CallbackTrait as "CallbackPtr", uid: u32 as "int"] {
                callback.on_user_left(uid)
            });
        }
    
        virtual void audioFrameReceived(unsigned int uid, const agora::linuxsdk::AudioFrame *frame) const {
            (void)uid;
            (void)frame;
        }
        virtual void videoFrameReceived(unsigned int uid, const agora::linuxsdk::VideoFrame *frame) const {
            (void)uid;
            (void)frame;
        }
        virtual void onActiveSpeaker(uid_t uid) {
            (void)uid;
        }
        virtual void onAudioVolumeIndication(const agora::linuxsdk::AudioVolumeInfo* speakers, unsigned int speakerNum) {
            (void)speakers;
            (void)speakerNum;
        }
    
        virtual void onFirstRemoteVideoDecoded(uid_t uid, int width, int height, int elapsed) {
            (void)uid;
            (void)width;
            (void)height;
            (void)elapsed;
        }
    
        virtual void onFirstRemoteAudioFrame(uid_t uid, int elapsed) {
            (void)uid;
            (void)elapsed;
        }
    
        virtual void onReceivingStreamStatusChanged(bool receivingAudio, bool receivingVideo) {
            (void)receivingAudio;
            (void)receivingVideo;
        }
    
        virtual void onConnectionLost() {}
    
        virtual void onConnectionInterrupted() {}
    
        virtual void onRejoinChannelSuccess(const char* channelId, uid_t uid) {
            (void)channelId;
            (void)uid;
        }
    
        virtual void onConnectionStateChanged(agora::linuxsdk::ConnectionStateType state, agora::linuxsdk::ConnectionChangedReasonType reason){
            (void)state;
            (void)reason;
        }
    
        virtual void onRecordingStats(const agora::linuxsdk::RecordingStats& stats){
            (void)stats;
        }
    
        virtual void onRemoteVideoStats(uid_t uid, const agora::linuxsdk::RemoteVideoStats& stats){
            (void)uid;
            (void)stats;
        }
    
        virtual void onRemoteAudioStats(uid_t uid, const agora::linuxsdk::RemoteAudioStats& stats){
            (void)uid;
            (void)stats;
        }
            
        virtual void onLocalUserRegistered(uid_t uid, const char* userAccount){
            (void)uid;
            (void)userAccount;
        }
    
        virtual void onUserInfoUpdated(uid_t uid, const agora::linuxsdk::UserInfo& info){
            (void)uid;
            (void)info;
        }
    };    
}}

pub trait RawPtr {
    fn raw_ptr(&self) -> *mut u32;
}

pub trait Emitter: RawPtr {
    fn set_callback(&mut self, callback: Box<dyn Listener>);
}

pub struct AgoraSdkEvents {
    rawptr: *mut u32,
    listener: Option<Box<dyn Listener>>,
}

pub trait Listener {
    fn error(&self, error: u32, stat_code: u32);
    fn joined(&mut self, uid: u32);
    fn left(&mut self, uid: u32);
    fn channel_joined(&mut self, channel: String, uid: u32);
}

impl CallbackTrait for AgoraSdkEvents {
    
    fn on_error(&mut self, error: u32, stat_code: u32) {
        if self.listener.is_some() {
            self.listener.as_mut().unwrap().error(error, stat_code);
        }
    }

    fn on_user_joined(&mut self, uid: u32) {
        if self.listener.is_some() {
            self.listener.as_mut().unwrap().joined(uid);
        }
    }

    fn on_user_left(&mut self, uid: u32) {
        if self.listener.is_some() {
            self.listener.as_mut().unwrap().left(uid);
        }
    }

    fn on_channel_join_success(&mut self, channel: &str, uid: u32) {
        if self.listener.is_some() {
            self.listener.as_mut().unwrap().channel_joined(channel.to_string(), uid);
        }
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
            listener: None,
        }
    }
}

impl RawPtr for AgoraSdkEvents {
    fn raw_ptr(&self) -> *mut u32 {
        return self.rawptr;
    }
}

impl Emitter for AgoraSdkEvents {

    fn set_callback(&mut self, callback: Box<dyn Listener>) {
        if self.listener.is_some() {
            return;
        }
        self.listener = Some(callback);
        
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

#[derive(Clone)]
pub struct AgoraSdk {
    sdk: *mut u32,
}

unsafe impl Send for AgoraSdk {}

pub trait IAgoraSdk: RawPtr + Send {
    fn set_handler<E: 'static + Emitter>(&mut self, emitter: &E);
    fn set_keep_last_frame(&self, keep : bool);
    fn create_channel(&self, app_id: &str, channel_key: &str, name: &str, uid: u32, config: &Config) -> bool;
    fn update_mix_mode_setting(&self, width: u32, height: u32, is_video_mix: bool);
    fn leave_channel(&self) -> bool;
    fn set_video_mixing_layout(&self, layout: &Layout) -> u32;
    fn release(&self) -> bool;
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
}

impl RawPtr for AgoraSdk {
    fn raw_ptr(&self) -> *mut u32 {
        return self.sdk;
    }
}

impl IAgoraSdk for AgoraSdk {
    fn set_handler<E: Emitter>(&mut self, emitter: &E) {
        unsafe {
            let handler = emitter.raw_ptr();
            let me = self.raw_ptr();
            cpp!([me as "agora::AgoraSdk*", handler as "agora::recording::IRecordingEngineEventHandler*"] {
                me->setHandler(handler);
            })
        };
    }

    fn set_keep_last_frame(&self, keep : bool) {
        let me = self.raw_ptr();
        unsafe {
            cpp!([me as "agora::AgoraSdk*", keep as "bool"] {
                    return me->setKeepLastFrame(keep);
                }
            )
        }
    }

    fn create_channel(&self, app_id: &str, channel_key: &str, name: &str, uid: u32, config: &Config) -> bool {
        let me = self.raw_ptr();
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
    
    fn update_mix_mode_setting(&self, width: u32, height: u32, is_video_mix: bool) {
        let me = self.raw_ptr();
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

    fn leave_channel(&self) -> bool {
        let me = self.raw_ptr();
        unsafe {
            cpp!([me as "agora::AgoraSdk*"] -> bool as "bool" {
                    return me->leaveChannel();
                }
            )
        }
    }

    fn set_video_mixing_layout(&self, layout: &Layout) -> u32 {
        let me = self.raw_ptr();
        unsafe {
            cpp!([  me as "agora::AgoraSdk*", 
                    layout as "agora::linuxsdk::VideoMixingLayout*"] -> u32 as "int" {
                
                return me->setVideoMixingLayout(*layout);
            })
        }        
    }

    fn release(&self) -> bool {
        let me = self.raw_ptr();
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
        let me = self.raw_ptr();
        unsafe {
            cpp!([me as "agora::AgoraSdk*"] {
                delete me;
            })
        };
    }    
}

pub fn agora_core_path() -> String {
    match env::var("AGORA_CORE_PATH") {
        Ok(path) => path,
        _ => "".to_string(),
    }
}

pub fn app_id() -> String {
    match env::var("APP_ID") {
        Ok(id) => id,
        _ => "".to_string(),
    }
}

pub fn channel() -> String {
    match env::var("CHANNEL") {
        Ok(channel) => channel,
        _ => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};
    use uuid::Uuid;
    use std::io::prelude::*;
    use std::fs::{self, File};
    use std::rc::Rc;
    use std::cell::RefCell;
    #[test]
    fn callback() {
        struct TestListener {}
        impl Listener for TestListener {
            fn error(&self, error: u32, stat_code: u32) {
                println!("on_error -> {} {}", error, stat_code);
                assert!(error == 0, "error received");
            }
            fn joined(&mut self, uid: u32) {
                println!("joined {}", uid);
            }
            fn left(&mut self, uid: u32) {
                println!("user left");
            }
            fn channel_joined(&mut self, channel: String, uid: u32) {
                println!("channel joined {} {}", channel, uid);
            }
        }

        let test: Option<Box<dyn Listener>> = Some(Box::new(TestListener{}));
        if let Some(mut b) = test {
            b.joined(100);
        }
    }
    
    #[test]
    fn recorder_create() {
        struct Recorder {
            sdk: Rc<RefCell<AgoraSdk>>,
        };

        struct RecorderListener {
            sdk: Rc<RefCell<AgoraSdk>>,
        }

        impl Listener for RecorderListener {
            fn error(&self, error: u32, stat_code: u32) {
                println!("on_error -> {} {}", error, stat_code);
                assert!(error == 0, "error received");
            }
            fn joined(&mut self, uid: u32) {
                // when we have a user record them as full in layout
                let layout = Layout::new();
                layout.set_regions(vec![Region::new(
                    uid, 0.0, 0.0, 1.0, 1.0, 1.0, 1 
                ),
                Region::new(
                    uid, 0.5, 0.5, 0.5, 0.5, 1.0, 1 
                )]);
    
                (*self.sdk.borrow_mut()).set_video_mixing_layout(&layout);
            }
            fn left(&mut self, uid: u32) {
                println!("user left");
            }
            fn channel_joined(&mut self, channel: String, uid: u32) {
                println!("channel joined {} {}", channel, uid);
            }
        }

        impl Recorder {
            pub fn new() -> Self {
                let sdk = AgoraSdk::new();
                let sdk = Rc::new(RefCell::new(sdk));

                Recorder {
                    sdk
                }
            }

            pub fn start(&mut self) {
                
                let mut events = AgoraSdkEvents::new();
                let callbacks = RecorderListener {
                    sdk: self.sdk.clone(),
                };
                events.set_callback(Box::new(callbacks));

                (*self.sdk.borrow_mut()).set_handler(&events);
        
                // Set up configuration file for recordings
                let path = agora_core_path();
                assert!(path != "", "AGORA_CORE_PATH not set!");
                let app_id = app_id();
                assert!(app_id != "", "APP_ID not set!");
                let channel = channel();
                assert!(channel != "", "CHANNEL not set!");
                let cwd = env::current_dir().expect("current working directory");
                let output = format!("{}/{}", cwd.display(), Uuid::new_v4());
                fs::create_dir(&output).expect("create directory for recordings");
                let json_cfg_contents = format!("{{\"Recording_Dir\":\"{}\"}}", output);
                let output_json_cfg = format!("{}/cfg.json", output);
                let mut file = File::create(&output_json_cfg).expect("create cfg.json for recordings");
                file.write_all(json_cfg_contents.as_bytes()).expect("write config contents");
        
                let config = Config::new();       
                config.set_config_path(&output_json_cfg);
                config.set_app_lite_dir(&path);
                config.set_mixing_enabled(true);
                config.set_mixed_video_audio(MixedAvCodecType::MixedAvCodecV2);
                config.set_idle_limit_sec(300);        
                config.set_channel_profile(ChannelProfile::LiveBroadcast);
                config.set_trigger_mode(TriggerMode::Automatic);
                config.set_mix_resolution(640, 480, 15, 500);        
                config.set_audio_indication_interval(0);
                
                // // At the moment we need to create a room called demo for this test
                assert!((*self.sdk.borrow_mut()).create_channel(&app_id, "", &channel, 0, &config));
                
                thread::sleep(time::Duration::from_millis(5000));
        
                // check we have generated an mp4 file
                let result = fs::read_dir(&output).expect("read output directory");
                let v : Vec<_> = result
                                    .filter_map(|r|r.ok()) // filter oks
                                    .map(|de|de.path())
                                    .filter(|p|p.is_file())
                                    .collect();
                
                let v : Vec<_> = v.iter()
                                .filter_map(|v|v.extension())
                                .filter(|ext|ext.to_str() == Some("mp4")).collect();
                
                fs::remove_dir_all(&output).expect("remove output directory");
                assert!(v.len() == 1, "no mp4 file created");
            }
        }

        let mut recorder = Recorder::new();
        recorder.start();
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

    #[test]
    fn region_new() {
        let uid = 1;
        let x = 1.0;
        let y = 1.0;
        let width = 1.0;
        let height = 1.0;
        let alpha = 1.0;
        let render_mode = 1;

        let region = Region::new(uid, x, y, width, height, alpha, render_mode);
        
        assert!(region.x() == x);
        assert!(region.y() == y);
        assert!(region.width() == width);
        assert!(region.height() == height);
        assert!(region.alpha() == alpha);
        assert!(region.render_mode() == render_mode);
    }

    #[test]
    fn layout_new() {
        let layout = Layout::new();
        layout.set_canvas_width(100);
        assert!(layout.canvas_width() == 100);
        layout.set_canvas_height(100);
        assert!(layout.canvas_height() == 100);
        layout.set_background_rgb("#ff0000");
        assert!(layout.background_rgb() == Ok("#ff0000"));
    }

    #[test]
    fn layout_set_regions() {
        let regions = vec![
            Region::new(1, 1.0, 1.0, 1.0, 1.0, 1.0, 1),
            Region::new(2, 1.0, 1.0, 1.0, 1.0, 1.0, 1),
            Region::new(3, 1.0, 1.0, 1.0, 1.0, 1.0, 1)
        ];

        let layout = Layout::new();
        layout.set_regions(regions);

        let get_regions = layout.get_regions();
        assert!(get_regions.len() == 3);
        assert!(get_regions[0].uid() == 1);
        assert!(get_regions[1].uid() == 2);
        assert!(get_regions[2].uid() == 3);
    }
}