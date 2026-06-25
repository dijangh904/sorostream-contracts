use soroban_sdk::{Address, Env, Symbol};

/// Emitted when a new stream is created.
pub fn stream_created(
    env: &Env,
    stream_id: u64,
    sender: &Address,
    recipient: &Address,
    amount: i128,
    flow_rate: i128,
    end_time: u64,
) {
    env.events().publish(
        (Symbol::new(env, "StreamCreated"), stream_id),
        (sender.clone(), recipient.clone(), amount, flow_rate, end_time),
    );
}

/// Emitted when a recipient withdraws claimable tokens.
pub fn stream_withdrawn(env: &Env, stream_id: u64, recipient: &Address, amount: i128, timestamp: u64) {
    env.events().publish(
        (Symbol::new(env, "StreamWithdrawn"), stream_id),
        (recipient.clone(), amount, timestamp),
    );
}

/// Emitted when a sender cancels a stream.
pub fn stream_cancelled(
    env: &Env,
    stream_id: u64,
    sender: &Address,
    refund_amount: i128,
    recipient_amount: i128,
) {
    env.events().publish(
        (Symbol::new(env, "StreamCancelled"), stream_id),
        (sender.clone(), refund_amount, recipient_amount),
    );
}

/// Emitted when a sender tops up an existing stream.
pub fn stream_topped_up(env: &Env, stream_id: u64, added_amount: i128, new_end_time: u64) {
    env.events().publish(
        (Symbol::new(env, "StreamToppedUp"), stream_id),
        (added_amount, new_end_time),
    );
}

/// Emitted when a stream naturally reaches its end time.
pub fn stream_completed(env: &Env, stream_id: u64) {
    env.events().publish(
        (Symbol::new(env, "StreamCompleted"), stream_id),
        (),
    );
}

/// Emitted when a stream's recipient is transferred to a new address.
pub fn stream_recipient_transferred(
    env: &Env,
    stream_id: u64,
    old_recipient: &Address,
    new_recipient: &Address,
) {
    env.events().publish(
        (Symbol::new(env, "RecipientTransferred"), stream_id),
        (old_recipient.clone(), new_recipient.clone()),
    );
}
