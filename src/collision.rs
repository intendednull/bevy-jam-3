pub mod group {
    use bevy_rapier2d::prelude::Group;

    pub static PLAYER_PROJECTILE: Group = Group::GROUP_2;
    pub static HOSTILE_PROJECTILE: Group = Group::GROUP_3;
    pub static HOSTILE: Group = Group::GROUP_4;
    pub static PLAYER: Group = Group::GROUP_5;
    pub static LOOT: Group = Group::GROUP_6;
}
