use bricc::realtime::rt_ctl::RtSystemControl;

pub struct DummyRtSystemControl {}

impl RtSystemControl for DummyRtSystemControl {
    fn wdt_subscribe_me() {}

    fn wdt_unsubscribe_me() {}

    fn wdt_feed_the_puppy() {}

    fn set_low_priority() {}
}
