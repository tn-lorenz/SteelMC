use steel_registry::vanilla_entities;

use super::Player;

#[test]
fn invalid_attack_targets_include_xp_orbs() {
    assert!(Player::is_invalid_attack_target(
        1,
        1,
        &vanilla_entities::PLAYER
    ));
    assert!(Player::is_invalid_attack_target(
        1,
        2,
        &vanilla_entities::ITEM
    ));
    assert!(Player::is_invalid_attack_target(
        1,
        2,
        &vanilla_entities::EXPERIENCE_ORB
    ));
    assert!(!Player::is_invalid_attack_target(
        1,
        2,
        &vanilla_entities::PIG
    ));
}
