use neurons::dendrite::{Dendrite, DendriteError, DendriteRestoreError, DendriteSnapshot};

fn snapshot(potential: i32) -> DendriteSnapshot {
    DendriteSnapshot {
        potential,
        threshold: 3,
        limit: 10,
    }
}

#[test]
fn restores_exact_state() {
    let original = snapshot(2);
    let restored = Dendrite::from_snapshot(original).unwrap();
    assert_eq!(restored.snapshot(), original);
}

#[test]
fn continues_processing_after_restore() {
    let mut restored = Dendrite::from_snapshot(snapshot(2)).unwrap();

    assert!(restored.receive(1));
    assert_eq!(restored.potential(), 0);

    assert!(!restored.receive(1));
    assert_eq!(restored.potential(), 1);
}

#[test]
fn rejects_potential_below_limit() {
    assert_eq!(
        Dendrite::from_snapshot(snapshot(-11)),
        Err(DendriteRestoreError::InvalidState)
    );
}

#[test]
fn rejects_potential_at_or_above_threshold() {
    for potential in [3, 10] {
        assert_eq!(
            Dendrite::from_snapshot(snapshot(potential)),
            Err(DendriteRestoreError::InvalidState)
        );
    }
}

#[test]
fn rejects_invalid_configuration() {
    let invalid = DendriteSnapshot {
        potential: 0,
        threshold: 0,
        limit: 10,
    };

    assert_eq!(
        Dendrite::from_snapshot(invalid),
        Err(DendriteRestoreError::Configuration(
            DendriteError::InvalidConfiguration
        ))
    );
}

#[test]
fn accepts_minimum_allowed_potential() {
    let restored = Dendrite::from_snapshot(snapshot(-10)).unwrap();
    assert_eq!(restored.potential(), -10);
}
