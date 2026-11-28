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

safe_index::new!(
    SharedShapeIndex,
    map: SharedShapes,
);

safe_index::new!(
    CuboidShapeIndex,
);

safe_index::new!(
    SphereShapeIndex,
);

safe_index::new!(
    CapsuleYShapeIndex,
);

safe_index::new!(
  CapsuleXShapeIndex,
);

safe_index::new!(
  TrimeshShapeIndex,
);

safe_index::new!(
    VoxelsShapeIndex,
);

safe_index::new!(
    HeightMapShapeIndex,
);