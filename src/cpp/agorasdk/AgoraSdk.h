#include <csignal>
#include <cstdint>
#include <iostream>
#include <sstream> 
#include <string>
#include <vector>
#include <algorithm>
#include <sys/time.h>
#include <unordered_map>
#include <unordered_set>
#include <set>

#include "IAgoraLinuxSdkCommon.h"
#include "IAgoraRecordingEngine.h"

//#include "base/atomic.h"
#include "base/opt_parser.h" 

namespace agora {

using std::string;
using std::cout;
using std::cerr;
using std::endl;

using agora::base::opt_parser;
using agora::linuxsdk::VideoFrame;
using agora::linuxsdk::AudioFrame;

enum LAYOUT_MODE_TYPE {
    DEFAULT_LAYOUT = 0,
    BESTFIT_LAYOUT = 1,
    VERTICALPRESENTATION_LAYOUT = 2,
};


struct MixModeSettings {
    int m_height;
    int m_width;
    bool m_videoMix;
    MixModeSettings():
        m_height(0),
        m_width(0),
        m_videoMix(false)
    {};
};

struct AudioFrameInfo {
    unsigned int m_channels;
    unsigned int m_index;
    AudioFrameInfo():
        m_channels(0),
        m_index(1)
    {};
};

// class AgoraSdkEvents :  virtual public agora::recording::IRecordingEngineEventHandler {
//     protected:
//     virtual void onError(int error, agora::linuxsdk::STAT_CODE_TYPE stat_code) {
//         //sdk->stoppedOnError();
//     }
//     virtual void onWarning(int warn) {
//     }
//     virtual void onJoinChannelSuccess(const char * channelId, agora::linuxsdk::uid_t uid) {
//     }
//     virtual void onLeaveChannel(agora::linuxsdk::LEAVE_PATH_CODE code) {
//     }

//     virtual void onUserJoined(agora::linuxsdk::uid_t uid, agora::linuxsdk::UserJoinInfos &infos) {
//     }

//     virtual void onRemoteVideoStreamStateChanged(agora::linuxsdk::uid_t uid, agora::linuxsdk::RemoteStreamState state, agora::linuxsdk::RemoteStreamStateChangedReason reason) {
//     }

//     virtual void onRemoteAudioStreamStateChanged(agora::linuxsdk::uid_t uid, agora::linuxsdk::RemoteStreamState state, agora::linuxsdk::RemoteStreamStateChangedReason reason) {
//     }

//     virtual void onUserOffline(agora::linuxsdk::uid_t uid, agora::linuxsdk::USER_OFFLINE_REASON_TYPE reason) {
//     }

//     virtual void audioFrameReceived(unsigned int uid, const agora::linuxsdk::AudioFrame *frame) const {
//     }
//     virtual void videoFrameReceived(unsigned int uid, const agora::linuxsdk::VideoFrame *frame) const {
//     }
//     virtual void onActiveSpeaker(uid_t uid) {
//     }
//     virtual void onAudioVolumeIndication(const agora::linuxsdk::AudioVolumeInfo* speakers, unsigned int speakerNum) {
//     }

//     virtual void onFirstRemoteVideoDecoded(uid_t uid, int width, int height, int elapsed) {
//     }

//     virtual void onFirstRemoteAudioFrame(uid_t uid, int elapsed) {}

//     virtual void onReceivingStreamStatusChanged(bool receivingAudio, bool receivingVideo) {}

//     virtual void onConnectionLost() {}

//     virtual void onConnectionInterrupted() {}

//     virtual void onRejoinChannelSuccess(const char* channelId, uid_t uid) {}

//     virtual void onConnectionStateChanged(agora::linuxsdk::ConnectionStateType state, agora::linuxsdk::ConnectionChangedReasonType reason){}

//     virtual void onRecordingStats(const agora::linuxsdk::RecordingStats& stats){}

//     virtual void onRemoteVideoStats(uid_t uid, const agora::linuxsdk::RemoteVideoStats& stats){}

//     virtual void onRemoteAudioStats(uid_t uid, const agora::linuxsdk::RemoteAudioStats& stats){}
        
//     virtual void onLocalUserRegistered(uid_t uid, const char* userAccount){}

//     virtual void onUserInfoUpdated(uid_t uid, const agora::linuxsdk::UserInfo& info){}
// }

class AgoraSdk {
    public:
        AgoraSdk(agora::recording::IRecordingEngineEventHandler * handler = nullptr);
        virtual ~AgoraSdk();
        virtual void setHandler(agora::recording::IRecordingEngineEventHandler * handler) {
            if (m_handler == nullptr) {
                m_handler = handler;
            }
        }

        virtual bool createChannel(const string &appid, const string &channelKey, const string &name,  agora::linuxsdk::uid_t uid,
                agora::recording::RecordingConfig &config);
        virtual bool createChannelWithUserAccount(const string &appid, const string &channelKey, const string &name,  const std::string& userAccount,
                agora::recording::RecordingConfig &config);

        virtual int setVideoMixLayout();
        virtual bool leaveChannel();
        virtual bool stoppedOnError();
        virtual bool release();
        virtual bool stopped() const;
        virtual void updateMixModeSetting(int width, int height, bool isVideoMix) {
            m_mixRes.m_width = width;
            m_mixRes.m_height = height;
            m_mixRes.m_videoMix = isVideoMix;
        }
        virtual const agora::recording::RecordingEngineProperties* getRecorderProperties();
        virtual void updateStorageDir(const char* dir) { m_storage_dir = dir? dir:"./"; }
        virtual void updateLayoutSetting(int layoutMode, int maxVertPreLayoutUid, const std::string& maxVertPreLayoutUserAccount) {
            m_layoutMode = static_cast<LAYOUT_MODE_TYPE >(layoutMode);
            m_maxVertPreLayoutUid = maxVertPreLayoutUid;
            m_maxVertPreLayoutUserAccount = maxVertPreLayoutUserAccount;
        }

        virtual int startService();
        virtual int stopService();

        virtual int setVideoMixingLayout(const agora::linuxsdk::VideoMixingLayout &layout);
        virtual agora::recording::RecordingConfig* getConfigInfo() { return &m_config;}
        virtual int setUserBackground(agora::linuxsdk::uid_t uid, const char* image_path);
        virtual void setLogLevel(agora::linuxsdk::agora_log_level level);
				void setMediaKeepTime(uint32_t time_ms);
        virtual int updateSubscribeVideoUids(uint32_t *uids, uint32_t num);
        virtual int updateSubscribeAudioUids(uint32_t *uids, uint32_t num);
        virtual uint32_t getUidByUserAccount(const char *userAccount);
        virtual uint32_t getUserAccountByUid(uint32_t uid, char* userAccountBuf, uint32_t buf_len);
        void setKeepLastFrame(bool keep);
        int updateWatermarkConfigs(uint32_t wm_num, linuxsdk::WatermarkConfig* config);
    
    private:
        void adjustDefaultVideoLayout(agora::linuxsdk::VideoMixingLayout::Region * regionList,
std::vector<agora::linuxsdk::uid_t>& subscribedUids);
        void adjustBestFitVideoLayout(agora::linuxsdk::VideoMixingLayout::Region * regionList,
    std::vector<agora::linuxsdk::uid_t>& subscribedUids);
        void adjustVerticalPresentationLayout(unsigned int maxResolutionUid, agora::linuxsdk::VideoMixingLayout::Region * regionList,
    std::vector<agora::linuxsdk::uid_t>& subscribedUids);

        void adjustVideo5Layout(unsigned int maxResolutionUid, agora::linuxsdk::VideoMixingLayout::Region * regionList,
    std::vector<agora::linuxsdk::uid_t>& subscribedUids);
        void adjustVideo7Layout(unsigned int maxResolutionUid, agora::linuxsdk::VideoMixingLayout::Region * regionList,
    std::vector<agora::linuxsdk::uid_t>& subscribedUids);
        void adjustVideo9Layout(unsigned int maxResolutionUid, agora::linuxsdk::VideoMixingLayout::Region * regionList,
    std::vector<agora::linuxsdk::uid_t>& subscribedUids);
        void adjustVideo17Layout(unsigned int maxResolutionUid, agora::linuxsdk::VideoMixingLayout::Region * regionList,
    std::vector<agora::linuxsdk::uid_t>& subscribedUids);

        void changeToVideo7Layout(unsigned int maxResolutionUid, agora::linuxsdk::VideoMixingLayout::Region * regionList,
    std::vector<agora::linuxsdk::uid_t>& subscribedUids);
        void changeToVideo9Layout(unsigned int maxResolutionUid, agora::linuxsdk::VideoMixingLayout::Region * regionList,
    std::vector<agora::linuxsdk::uid_t>& subscribedUids);
        void changeToVideo17Layout(unsigned int maxResolutionUid, agora::linuxsdk::VideoMixingLayout::Region * regionList,
    std::vector<agora::linuxsdk::uid_t>& subscribedUids);

        void adjustBestFitLayout_2(agora::linuxsdk::VideoMixingLayout::Region * regionList,
    std::vector<agora::linuxsdk::uid_t>& subscribedUids);
        void adjustBestFitLayout_Square(agora::linuxsdk::VideoMixingLayout::Region * regionList, int nSquare,
    std::vector<agora::linuxsdk::uid_t>& subscribedUids);
        void adjustBestFitLayout_17(agora::linuxsdk::VideoMixingLayout::Region * regionList,
    std::vector<agora::linuxsdk::uid_t>& subscribedUids);
        void setMaxResolutionUid(int number, unsigned int maxResolutionUid, agora::linuxsdk::VideoMixingLayout::Region * regionList, double weight_ratio);
	uint32_t now_s() const;
    
        agora::recording::IRecordingEngineEventHandler * m_handler;
        bool m_stopped;
        std::vector<agora::linuxsdk::uid_t> m_peers;
        std::string m_logdir;
        std::string m_storage_dir;
        MixModeSettings m_mixRes;
        agora::recording::RecordingConfig m_config;
        agora::recording::IRecordingEngine *m_engine;
        agora::linuxsdk::agora_log_level m_level;
        LAYOUT_MODE_TYPE m_layoutMode;
        int m_maxVertPreLayoutUid;
        std::string m_maxVertPreLayoutUserAccount;
        bool m_receivingAudio;
        bool m_receivingVideo;
        uint32_t m_mediaKeepTime;
        mutable uint32_t m_lastAudioKeepTime;
        mutable uint32_t m_lastVideoKeepTime;
        mutable std::unordered_map<unsigned int, AudioFrameInfo> m_audioFrameMap;
        std::unordered_set<uint32_t> m_subscribedVideoUids;
        std::unordered_set<uint32_t> m_subscribedAudioUids;
        std::set<std::string> m_subscribeVideoUserAccount;
        std::set<std::string> m_subscribeAudioUserAccount;
        bool m_keepLastFrame;
        std::string m_userAccount;
};


}
