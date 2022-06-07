use role_playing_game::Player;

fn main() {
    let mut no_mana_wizard = Player {
        health: 56,
        mana: Some(2),
        level: 22,
    };

    // we want to clone so we can compare before-and-after effects of casting the spell,
    // but we don't want to introduce that concept to the student yet, so we have to do it manually
    let clone = Player { ..no_mana_wizard };

    assert_eq!(no_mana_wizard.cast_spell(3), 0);
    assert_eq!(no_mana_wizard.health, clone.health);
    assert_eq!(no_mana_wizard.mana, clone.mana);
    assert_eq!(no_mana_wizard.level, clone.level);
}
