use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use tokio::sync::oneshot;
use std::marker::Send;

type Cancel = ();

pub fn make_debounce<T: Send + 'static>(
    delay: u64,
    func: impl Fn(T) + Send + Clone + 'static
) -> impl Fn(T) + Send  + Clone {
    let sleep_duration = Duration::from_millis(delay);
    let cancel_last = Arc::new(Mutex::new(None::<oneshot::Sender<Cancel>>));

    return move |arg| {
        let duration = sleep_duration;
        let (tx, rx) = oneshot::channel::<Cancel>();
        {
            let mut cancel_locked  = cancel_last.lock().expect("Should lock");
            let cancel_opt = cancel_locked.take();
            if let Some(channel) = cancel_opt{
                channel.send(()).ok();
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
