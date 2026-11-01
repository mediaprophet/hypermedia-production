//! Pure lease-state regressions. No host/device/stream is opened.
use super::*;

#[test]
fn never_started_preparation_failure_releases_the_actual_guard() {
    let owner=AtomicU8::new(DeviceOwnership::Free as u8);
    let ack=Arc::new(AtomicBool::new(false));
    let first=DeviceSlot::acquire(&owner,ack.clone()).unwrap();
    assert!(DeviceSlot::acquire(&owner,ack.clone()).is_err());
    drop(first);
    assert_eq!(owner.load(Ordering::Acquire),DeviceOwnership::Free as u8);
    assert!(DeviceSlot::acquire(&owner,ack).is_ok());
}

#[test]
fn failed_play_without_reset_ack_poison_blocks_reacquisition() {
    let owner=AtomicU8::new(DeviceOwnership::Free as u8);
    let ack=Arc::new(AtomicBool::new(false));
    let mut guard=DeviceSlot::acquire(&owner,ack.clone()).unwrap();
    guard.before_play(); // Stream::play may fail after partial backend startup.
    drop(guard);
    assert_eq!(owner.load(Ordering::Acquire),DeviceOwnership::Poisoned as u8);
    let error=DeviceSlot::acquire(&owner,ack).err().unwrap();
    assert!(error.contains("poisoned"));
}

#[test]
fn acknowledged_reset_releases_but_a_later_play_invalidates_old_ack() {
    let owner=AtomicU8::new(DeviceOwnership::Free as u8);
    let ack=Arc::new(AtomicBool::new(false));
    let mut guard=DeviceSlot::acquire(&owner,ack.clone()).unwrap();
    guard.before_play();
    ack.store(true,Ordering::Release); // Only actual successful backend reset in production.
    drop(guard);
    let mut next=DeviceSlot::acquire(&owner,ack.clone()).unwrap();
    next.before_play();
    assert!(!ack.load(Ordering::Acquire));
    drop(next);
    assert_eq!(owner.load(Ordering::Acquire),DeviceOwnership::Poisoned as u8);
}

#[test]
fn panic_after_play_attempt_poison_is_retained() {
    let owner=AtomicU8::new(DeviceOwnership::Free as u8);
    let ack=Arc::new(AtomicBool::new(false));
    let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut guard=DeviceSlot::acquire(&owner,ack.clone()).unwrap();
        guard.before_play();
        panic!("simulated worker/backend unwind before reset acknowledgement");
    }));
    assert!(result.is_err());
    assert!(DeviceSlot::acquire(&owner,ack).is_err());
    assert_eq!(owner.load(Ordering::Acquire),DeviceOwnership::Poisoned as u8);
}
