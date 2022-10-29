//! All the selectors used by this crate.

use objc4::selector;

/// Defines all the selectors used by this crate.
#[macro_export]
macro_rules! all_selectors {
    () => {
        selector!(BOOLVALUE = "boolValue");
        selector!(CHARVALUE = "charValue");
        selector!(COPY = "copy");
        selector!(COUNT = "count");
        selector!(DOUBLEVALUE = "doubleValue");
        selector!(FLOATVALUE = "floatValue");
        selector!(INITWITHBYTES_LENGTH_ENCODING_ = "initWithBytes:length:encoding:");
        selector!(INTEGERVALUE = "integerValue");
        selector!(INTVALUE = "intValue");
        selector!(ISEQUALTOSTRING_ = "isEqualToString:");
        selector!(LENGTH = "length");
        selector!(LONGLONGVALUE = "longLongValue");
        selector!(NUMBERWITHBOOL_ = "numberWithBool:");
        selector!(NUMBERWITHCHAR_ = "numberWithChar:");
        selector!(NUMBERWITHDOUBLE_ = "numberWithDouble:");
        selector!(NUMBERWITHFLOAT_ = "numberWithFloat:");
        selector!(NUMBERWITHINTEGER_ = "numberWithInteger:");
        selector!(NUMBERWITHINT_ = "numberWithInt:");
        selector!(NUMBERWITHLONGLONG_ = "numberWithLongLong:");
        selector!(NUMBERWITHSHORT_ = "numberWithShort:");
        selector!(NUMBERWITHUNSIGNEDCHAR_ = "numberWithUnsignedChar:");
        selector!(NUMBERWITHUNSIGNEDINTEGER_ = "numberWithUnsignedInteger:");
        selector!(NUMBERWITHUNSIGNEDINT_ = "numberWithUnsignedInt:");
        selector!(NUMBERWITHUNSIGNEDLONGLONG_ = "numberWithUnsignedLongLong:");
        selector!(NUMBERWITHUNSIGNEDSHORT_ = "numberWithUnsignedShort:");
        selector!(OBJECTFORKEY_ = "objectForKey:");
        selector!(REMOVEOBJECTFORKEY_ = "removeObjectForKey:");
        selector!(SETOBJECT_FORKEY_ = "setObject:forKey:");
        selector!(SHORTVALUE = "shortValue");
        selector!(UNSIGNEDCHARVALUE = "unsignedCharValue");
        selector!(UNSIGNEDINTEGERVALUE = "unsignedIntegerValue");
        selector!(UNSIGNEDINTVALUE = "unsignedIntValue");
        selector!(UNSIGNEDLONGLONGVALUE = "unsignedLongLongValue");
        selector!(UNSIGNEDSHORTVALUE = "unsignedShortValue");
        selector!(UTF8STRING = "UTF8String");
    };
}

objc4::all_selectors!();
all_selectors!();
