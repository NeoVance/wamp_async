use tokio::task;
use core::{
    future::Future,
    pin::Pin,
    task::{ Context, Poll },
};
use crate::Client;
use crate::WampId;
use crate::WampError;
use tokio::task::JoinError;
use crate::CancelOptions;
use crate::OptionBuilder;

/// Spawn a new tokio Task and cancel it on drop.
pub fn spawn<'a, T>(request: WampId, client: &'a mut Client<'a>, future: T) -> WampCallResult<'a, T::Output>
where
    T: Future + Send + 'a,
    T::Output: Send + 'a,
{
    WampCallResult(task::spawn(future), request, client)
}

/// Cancels the wrapped tokio Task on Drop.
pub struct WampCallResult<'a, T>(pub task::JoinHandle<T>, pub WampId, &'a mut Client<'a>);

impl<'a, T> WampCallResult<'a, T> {
    pub async fn cancel(&self, options: CancelOptions) -> Result<WampId, WampError> {
        match self.2.cancel(self.1, match options.get_dict() {
            Some(options) => options,
            None => CancelOptions::default().get_dict().unwrap(),
        }).await {
            Ok(id) => {
                unsafe {
                    drop(self);
                };
                
                Ok(id)
            }
            Err(e) => {
                unsafe {
                    drop(self);
                };
                return Err(From::from(format!(
                    "Request Cancellation : {}",
                    e
                )));
            }
        }
    }
}

impl<'a, T> Future for WampCallResult<'a, T>{
    type Output = Result<T, JoinError>;
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe { Pin::new_unchecked(&mut self.0) }.poll(cx)
    }
}

impl<'a, T> Drop for WampCallResult<'a, T> {
    fn drop(&mut self) {
        self.0.abort();
    }
}
