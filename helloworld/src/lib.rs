/// First line is a short summary describing function.
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. Assume we're testing `doccomments` crate:
///
/// ```
/// let result = helloworld::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

use std::collections::HashMap;

// error types
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum SubscriberError {
    ExampleError1,
    ExampleError2
}

/// An event type.
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Event {
    StartRequest,
    EndRequest,
    PassRequest,
    FailRequest(SubscriberError)
}

/// A subscriber (listener) has type of a callable function.
pub type Subscriber = fn(event_type: Event);

/// Publisher sends events to subscribers (listeners).
#[derive(Default)]
pub struct Publisher {
    events: HashMap<Event, Vec<Subscriber>>,
}

impl Publisher {
    pub fn subscribe(&mut self, event_type: Event, listener: Subscriber) {
        self.events.entry(event_type.clone()).or_default();
        self.events.get_mut(&event_type).unwrap().push(listener);
    }

    pub fn unsubscribe(&mut self, event_type: Event, listener: Subscriber) {
        self.events
            .get_mut(&event_type)
            .unwrap()
            .retain(|&x| x != listener);
    }

    pub fn notify(&self, event_type: Event) {
        let listeners = self.events.get(&event_type).unwrap();
        for listener in listeners {
            listener(event_type);
        }
    }

    pub fn do_pass(&self) {
        self.notify(Event::StartRequest);
        self.notify(Event::EndRequest);
        self.notify(Event::PassRequest);
    }

    pub fn do_fail1(&self) {
        self.notify(Event::StartRequest);
        self.notify(Event::EndRequest);
        self.notify(Event::FailRequest(SubscriberError::ExampleError1));
    }

    pub fn do_fail2(&self) {
        self.notify(Event::StartRequest);
        self.notify(Event::EndRequest);
        self.notify(Event::FailRequest(SubscriberError::ExampleError2));
    }
}

/// Worker has its own logic and it utilizes a publisher
/// to operate with subscribers and events.
#[derive(Default)]
pub struct Worker {
    publisher: Publisher,
}

impl Worker {
    pub fn test_pass(&self) {
        self.publisher.do_pass();
    }

    pub fn test_fail1(&self) {
        self.publisher.do_fail1();
    }

    pub fn test_fail2(&self) {
        self.publisher.do_fail2();
    }

    pub fn events(&mut self) -> &mut Publisher {
        &mut self.publisher
    }
}

fn event_handler(event_type: Event) {
    match event_type {
        Event::StartRequest => println!("Event type Start"),
        Event::EndRequest => println!("Event type End"),
        Event::PassRequest => println!("Event type Pass"),
        Event::FailRequest(error) => match error {
            SubscriberError::ExampleError1 => println!("Fail Request error1"),
            SubscriberError::ExampleError2 => println!("Fail Request error2"),
        }
    }
}

pub fn observer() {
    let mut worker = Worker::default();
    worker.events().subscribe(Event::StartRequest, event_handler);
    worker.events().subscribe(Event::EndRequest, event_handler);
    worker.events().subscribe(Event::PassRequest, event_handler);
    worker.events().subscribe(Event::FailRequest(SubscriberError::ExampleError1), event_handler);
    worker.events().subscribe(Event::FailRequest(SubscriberError::ExampleError2), event_handler);
    worker.test_pass();
    worker.test_fail1();
    worker.events().unsubscribe(Event::StartRequest, event_handler);
    worker.events().unsubscribe(Event::PassRequest, event_handler);
    worker.events().unsubscribe(Event::FailRequest(SubscriberError::ExampleError1), event_handler);
    worker.test_fail2();

    worker.events().unsubscribe(Event::EndRequest, event_handler);
    worker.events().unsubscribe(Event::FailRequest(SubscriberError::ExampleError2), event_handler);
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_observer() {
        observer();
    }
}
