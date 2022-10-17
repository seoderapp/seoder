use crate::json;
use crate::string_concat::string_concat;
use crate::string_concat::string_concat_impl;
use crate::tokio;

use crate::ENTRY_PROGRAM;
use futures_util::SinkExt;
use tungstenite::Message;

use crate::OutGoing;

/// remove file from download paths
pub async fn remove_file(mut outgoing: OutGoing, input: &str) -> OutGoing {
    tokio::fs::remove_file(string_concat!(&ENTRY_PROGRAM.1, &input))
        .await
        .unwrap();

    let v = json!({ "dfpath": input });

    outgoing
        .send(Message::Text(v.to_string()))
        .await
        .unwrap_or_default();

    outgoing
}
