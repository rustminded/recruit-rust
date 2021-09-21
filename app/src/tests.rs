#[test]
fn normal_offset_range_with_different_main_range() {
    let timezone = chrono::Duration::hours(6);

    let first_range = crate::utc_offset::UtcOffsetRange::new(timezone, 2);
    assert_eq!(first_range.start(timezone).num_hours(), 4);
    assert_eq!(first_range.end(timezone).num_hours(), 8);

    let second_range = crate::utc_offset::UtcOffsetRange::new(timezone, 3);
    assert_eq!(second_range.start(timezone).num_hours(), 3);
    assert_eq!(second_range.end(timezone).num_hours(), 9);

    let third_range = crate::utc_offset::UtcOffsetRange::new(timezone, 4);
    assert_eq!(third_range.start(timezone).num_hours(), 2);
    assert_eq!(third_range.end(timezone).num_hours(), 10);
}

#[test]
fn positive_offset_range_with_different_main_range() {
    let timezone = chrono::Duration::hours(14);

    let first_range = crate::utc_offset::UtcOffsetRange::new(timezone, 2);
    assert_eq!(first_range.start(timezone).num_hours(), 12);
    assert_eq!(first_range.end(timezone).num_hours(), -10);

    let second_range = crate::utc_offset::UtcOffsetRange::new(timezone, 3);
    assert_eq!(second_range.start(timezone).num_hours(), 11);
    assert_eq!(second_range.end(timezone).num_hours(), -9);

    let third_range = crate::utc_offset::UtcOffsetRange::new(timezone, 4);
    assert_eq!(third_range.start(timezone).num_hours(), 10);
    assert_eq!(third_range.end(timezone).num_hours(), -8);
}

#[test]
fn negative_offset_range_with_different_main_range() {
    let timezone = chrono::Duration::hours(-12);

    let first_range = crate::utc_offset::UtcOffsetRange::new(timezone, 2);
    assert_eq!(first_range.start(timezone).num_hours(), -10);
    assert_eq!(first_range.end(timezone).num_hours(), 12);

    let second_range = crate::utc_offset::UtcOffsetRange::new(timezone, 3);
    assert_eq!(second_range.start(timezone).num_hours(), -9);
    assert_eq!(second_range.end(timezone).num_hours(), 11);

    let third_range = crate::utc_offset::UtcOffsetRange::new(timezone, 4);
    assert_eq!(third_range.start(timezone).num_hours(), -8);
    assert_eq!(third_range.end(timezone).num_hours(), 10);
}
