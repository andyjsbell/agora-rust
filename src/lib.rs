use cpp::cpp;

cpp!{{
    #include "src/cpp/agorasdk/AgoraSdk.h"
}}

struct Recorder {
    id: String
}

impl Recorder {
    pub fn new() -> Recorder {
        
        let r = unsafe {
            cpp!([] {
                agora::AgoraSdk recorder;
            })
        };
        
        Recorder {
            id: String::from("test")
        }   
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let recorder = Recorder::new();
        assert_eq!(0, 1);
    }
}