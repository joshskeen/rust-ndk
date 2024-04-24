use android_activity::AndroidApp;
use log::info;

#[no_mangle]
fn android_main(_app: AndroidApp) {
    info!("hello world");
    println!("hello world");
}