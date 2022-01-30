use bricc::realtime::rt_ctl::RtSystemControl;
use esp_idf_sys::c_types::c_uint;
use std::ptr::null_mut;

pub struct EspRtSystemControl {}

impl RtSystemControl for EspRtSystemControl {
    fn wdt_subscribe_me() {
        if esp_idf_sys::esp!(unsafe { esp_idf_sys::esp_task_wdt_status(null_mut()) }).is_err() {
            let result = esp_idf_sys::esp!(unsafe { esp_idf_sys::esp_task_wdt_add(null_mut()) });
            if result.is_err() {
                println!("Failed to subscribe task to WDT: {}", result.unwrap_err());
                panic!();
            }
        } else {
            // println!("Task already subscribed to WDT");
        }
    }

    fn wdt_unsubscribe_me() {
        if esp_idf_sys::esp!(unsafe { esp_idf_sys::esp_task_wdt_status(null_mut()) }).is_ok() {
            let result = esp_idf_sys::esp!(unsafe { esp_idf_sys::esp_task_wdt_delete(null_mut()) });
            if result.is_err() {
                println!(
                    "Failed to unsubscribe task from WDT: {}",
                    result.unwrap_err()
                );
                panic!();
            }
        } else {
            // println!("Task already unsubscribed from WDT");
        }
    }

    fn wdt_feed_the_puppy() {
        if esp_idf_sys::esp!(unsafe { esp_idf_sys::esp_task_wdt_status(null_mut()) }).is_ok() {
            if esp_idf_sys::esp!(unsafe { esp_idf_sys::esp_task_wdt_reset() }).is_err() {
                println!("Failed to feed the WDT");
                panic!();
            }
        } else {
            println!("Not feeding WDT: Not subscribed");
        }
    }

    fn set_low_priority() {
        unsafe {
            esp_idf_sys::vTaskPrioritySet(null_mut(), 1u32 as c_uint);
        }
    }
}
