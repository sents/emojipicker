use std::marker::Send;
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
use tokio::time::{sleep, Duration};

#[derive(Debug)]
struct Cancel();

pub fn make_debounce<T: Send + 'static>(
    delay: u64,
    func: impl Fn(T) + Send + Clone + 'static,
) -> impl Fn(T) + Send + Clone {
    let sleep_duration = Duration::from_millis(delay);
    let cancel_last = Arc::new(Mutex::new(None::<oneshot::Sender<Cancel>>));

    move |arg| {
        let duration = sleep_duration;
        let (tx, rx) = oneshot::channel::<Cancel>();
        {
            let mut cancel_locked = cancel_last
                .lock()
                .expect("Could not lock debounce channel. Possible panic in other thread.");
            if let Some(channel) = cancel_locked.take() {
                channel.send(Cancel()).ok();
            }
            cancel_locked.replace(tx);
        }

        let cloned_func = func.clone();
        let _ = tokio::spawn(async move {
            tokio::select! {
                _ = sleep(duration) => {
                    cloned_func(arg)
                }
                _  = rx => {}
            }
        });
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Debouncer {
    delay: Duration,
    last_call: Arc<Mutex<Option<oneshot::Sender<Cancel>>>>,
}

#[allow(dead_code)]
impl Debouncer {
    pub fn new(delay: u64) -> Self {
        Debouncer {
            delay: Duration::from_millis(delay),
            last_call: Arc::new(Mutex::new(None)),
        }
    }
    pub fn call(&self, f: impl FnOnce() + Send + 'static) {
        let (tx, rx) = oneshot::channel::<Cancel>();
        {
            let mut locked = self
                .last_call
                .lock()
                .expect("Could not lock debounce channel. Possible panic in other thread.");
            if let Some(channel) = locked.take() {
                channel.send(Cancel()).ok();
            }
            locked.replace(tx);
        }

        let cloned_delay = self.delay.clone();
        tokio::spawn(async move {
            tokio::select! {
                _ = sleep(cloned_delay) => {
                    f()
                }
                _  = rx => {}
            }
        });
    }
}
