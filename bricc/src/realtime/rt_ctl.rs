pub trait RtSystemControl {
    fn wdt_subscribe_me();
    fn wdt_unsubscribe_me();
    fn wdt_feed_the_puppy();

    fn set_low_priority();
}
