use std::{sync::{Mutex, Arc}, future::Future, collections::HashMap, task::{Waker, Poll}};

pub struct CompletionFuture<T> 
    where T:Clone {
    id: u32,
    state: Arc<Mutex<FutureState<T>>>
}

struct FutureState<T> {
    pub state: u32,
    pub completion: Option<T>,
    pub wakers: HashMap<u32, Waker>
}

impl<T> CompletionFuture<T> 
    where T:Clone {
    pub(crate) fn new(state: Arc<Mutex<FutureState<T>>>, id: u32) -> CompletionFuture<T> {
        CompletionFuture { state, id }
    }
}

impl<T> Future for CompletionFuture<T>
    where T:Clone {
    type Output = T;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        if let Ok(mut state) = self.state.lock() {
            if let Some(result) = state.completion.clone() {
                return Poll::Ready(result);
            }else{
                state.wakers.remove(&self.id);
                state.wakers.insert(self.id.clone(), cx.waker().clone());
                return Poll::Pending;
            }
        }

        panic!("Cannot acquire lock on FutureState")
    }
}

pub struct FutureCompletion<T>
    where T:Clone {
    future_state: Arc<Mutex<FutureState<T>>>
}

impl<T> FutureCompletion<T>
    where T:Clone {
    pub fn new() -> FutureCompletion<T> {
        FutureCompletion { future_state: Arc::new(Mutex::new(FutureState { completion: None, state: 0, wakers: HashMap::new() }))}
    }

    pub fn get_future(&mut self) -> CompletionFuture<T> {
        if let Ok(mut state) = self.future_state.lock() {
            state.state += 1;
            return CompletionFuture::new(self.future_state.clone(), state.state);
        }

        panic!("Cannot acquire lock on FutureState")
    }

    pub fn set_result(&mut self, result: T) {
        if let Ok(mut state) = self.future_state.lock() {
            state.completion = Some(result);
            let wakers = state.wakers.to_owned();
            drop(state);

            for waker in wakers {
                waker.1.wake();
            }

            return;
        }

        panic!("Cannot acquire lock on FutureState")
    }
}
