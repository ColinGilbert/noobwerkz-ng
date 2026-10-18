safe_index::new!(
    PlayerChatMessagesIndex,
    btree map: PlayerChatMessages
);

// safe_index::new!(
//     ChatMessagesByTimeIndex,
//     btree map: ChatMessagesByTime
// );

safe_index::new!(
    Player3DUIDIndex,
    btree map: Player3DUIDsMap
);

safe_index::new!(
    NPC3DUIDIndex,
    btree map: NPC3DUIDsMap
);

safe_index::new!(
    Item3DUIDIndex,
    btree map: Item3DUIDsMap
);

safe_index::new!(
    Item3DTypeUIDIndex,
    btree map: Item3DTypeUIDsMap,
);