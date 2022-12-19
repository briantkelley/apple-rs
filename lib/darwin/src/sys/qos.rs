use crate::_sys::sys::qos::{
    QOS_CLASS_BACKGROUND, QOS_CLASS_DEFAULT, QOS_CLASS_USER_INITIATED, QOS_CLASS_USER_INTERACTIVE,
    QOS_CLASS_UTILITY,
};

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
#[repr(u32)]
pub enum Class {
    UserInteractive = QOS_CLASS_USER_INTERACTIVE,
    UserInitiated = QOS_CLASS_USER_INITIATED,
    Default = QOS_CLASS_DEFAULT,
    Utility = QOS_CLASS_UTILITY,
    Background = QOS_CLASS_BACKGROUND,
}

impl Default for Class {
    fn default() -> Self {
        Self::Default
    }
}

impl From<Class> for u32 {
    fn from(class: Class) -> Self {
        class as Self
    }
}
