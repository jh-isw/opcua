use prelude::*;

const DEFAULT_LIFETIME_COUNT: UInt32 = 300;
const DEFAULT_KEEPALIVE_COUNT: UInt32 = 100;

fn make_subscription(state: SubscriptionState) -> Subscription {
    let subscription_interval = 1000f64;
    let mut result = Subscription::new(0, true, subscription_interval, DEFAULT_LIFETIME_COUNT, DEFAULT_KEEPALIVE_COUNT, 0);
    result.state = state;
    result
}

fn make_publish_request_queue() -> Vec<PublishRequest> {
    vec!(
        PublishRequest {
            request_header: RequestHeader::new(&NodeId::null(), &DateTime::now(), 1),
            subscription_acknowledgements: None,
        }
    )
}

#[test]
fn basic_subscription() {
    let s =  Subscription::new(0, true, 1000f64, DEFAULT_LIFETIME_COUNT, DEFAULT_KEEPALIVE_COUNT, 0);
    assert!(s.state == SubscriptionState::Creating);
}

#[test]
fn update_state_3() {
    let mut s = make_subscription(SubscriptionState::Creating);
    let mut publish_requests: Vec<PublishRequest> = vec!();

    // Test #3 - state changes from Creating -> Normal
    let publishing_timer_expired = false;
    let (handled_state, action) = s.update_state(&mut publish_requests, publishing_timer_expired);

    assert_eq!(handled_state, 3);
    assert_eq!(action, UpdateStateAction::None);
    assert_eq!(s.message_sent, false);
    assert_eq!(s.state, SubscriptionState::Normal);
}

#[test]
fn update_state_4() {
    // Test #4 -
    // Create a subscription in the normal state, and an incoming publish request. Tick on a subscription
    // with no changes and ensure the request is still queued afterwards

    let mut s = make_subscription(SubscriptionState::Normal);
    let mut publish_requests = make_publish_request_queue();

    // Receive Publish Request
    //    &&
    //    (
    //        PublishingEnabled == FALSE
    //            ||
    //            (PublishingEnabled == TRUE
    //                && MoreNotifications == FALSE)
    //    )

    s.publishing_enabled = false;

    let publishing_timer_expired = false;
    let (handled_state, action) = s.update_state(&mut publish_requests, publishing_timer_expired);

    assert_eq!(handled_state, 4);
    assert_eq!(action, UpdateStateAction::None);
    assert_eq!(s.state, SubscriptionState::Normal);
    assert_eq!(publish_requests.len(), 1);

    // TODO repeat with publishing enabled true, more notifications false
}

#[test]
fn update_state_5() {
    // Test #5
    // Queue a publish request, publishing on, more notifications.
    // Ensure return notifications action

    let mut s = make_subscription(SubscriptionState::Normal);
    let mut publish_requests = make_publish_request_queue();

    // queue publish request
    // set publish enabled true
    // set more notifications true

    s.publishing_enabled = true;
    s.more_notifications = true;
    s.lifetime_counter = 1;

    let publishing_timer_expired = false;
    let (handled_state, action) = s.update_state(&mut publish_requests, publishing_timer_expired);

    assert_eq!(handled_state, 5);
    assert_eq!(action, UpdateStateAction::ReturnNotifications);
    assert_eq!(s.lifetime_counter, DEFAULT_LIFETIME_COUNT);
    assert_eq!(s.state, SubscriptionState::Normal);
    assert_eq!(s.message_sent, true);
    // TOD oensure deleted acknowledged notification msgs
}

#[test]
fn update_state_6() {
    // set publishing timer expires
    // set publishing requ queued
    // set publishing enabled true
    // set notifications available true

    let mut s = make_subscription(SubscriptionState::Normal);

    let mut publish_requests = vec![];
    s.publishing_enabled = true;
    s.notifications_available = true;
    s.lifetime_counter = 3; // Expect this to be reset
    s.publishing_req_queued = true;


    println!("state = {:#?}", s);

    let publishing_timer_expired = true;
    let (handled_state, action) = s.update_state(&mut publish_requests, publishing_timer_expired);


    println!("state after = {:#?}", s);

    // ensure 6
    assert_eq!(handled_state, 6);
    assert_eq!(action, UpdateStateAction::ReturnNotifications);
    assert_eq!(s.lifetime_counter, DEFAULT_LIFETIME_COUNT);
    assert_eq!(publish_requests.len(), 0);
    assert_eq!(s.message_sent, true);
}

#[test]
fn update_state_7() {
    // set timer expires
    // publishing request queued true
    // message sent true
    // publishing enabled false


    // ensure 7
    // ensure lifetime counter
    // ensure publishing request dequeued
    // ensure ReturnKeepAlive action
    // ensure message_sent true

    // Repeat with publishing enabled true and notifications available false
}

#[test]
fn update_state_8() {
    // set timer expires
    // set publishing request queued false
    // set message_sent false

    // ensure 8
    // ensure start publishing timer
}

#[test]
fn update_state_9() {
    // x
}

#[test]
fn update_state_10() {
    // x
}

#[test]
fn update_state_11() {
    // x
}

#[test]
fn update_state_12() {
    // x
}

#[test]
fn update_state_13() {
    // x
}

#[test]
fn update_state_14() {
    // x
}

#[test]
fn update_state_15() {
    // x
}

#[test]
fn update_state_16() {
    // x
}

#[test]
fn update_state_17() {
    // x
}