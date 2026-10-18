safe_index::new!(
    ServerScene3DIndex,
    map: ServerScenes3D
);

safe_index::new!(
    ServerItem3DIndex,
    map: ServerItem3Ds,
    btree set: ServerItems3DSet,
);

safe_index::new!(
    ServerPlayer3DIndex,
    map: ServerPlayers3D,
    btree set: ServerPlayers3DSet,
);

safe_index::new!(
    ServerNPC3DIndex,
    map: ServerNPCs3D,
    btree set: ServerNPCs3DSet,
);