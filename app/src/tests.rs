use crate::utc_offset::UtcOffsetRange;
use chrono::Duration;

// UtcOffsetRange
#[test]
fn normal_offset_range_with_different_main_range() {
    let timezone = Duration::hours(6);

    let first_range = UtcOffsetRange::new(timezone, 2);
    assert_eq!(first_range.primary.start().num_hours(), 4);
    assert_eq!(first_range.primary.end().num_hours(), 8);
    assert!(first_range.secondary.is_none());

    let second_range = UtcOffsetRange::new(timezone, 3);
    assert_eq!(second_range.primary.start().num_hours(), 3);
    assert_eq!(second_range.primary.end().num_hours(), 9);
    assert!(second_range.secondary.is_none());

    let third_range = UtcOffsetRange::new(timezone, 4);
    assert_eq!(third_range.primary.start().num_hours(), 2);
    assert_eq!(third_range.primary.end().num_hours(), 10);
    assert!(second_range.secondary.is_none());
}

#[test]
fn positive_offset_range_with_different_main_range() {
    let timezone = Duration::hours(14);
    let first_range = UtcOffsetRange::new(timezone, 2);
    let second_range = UtcOffsetRange::new(timezone, 3);
    let third_range = UtcOffsetRange::new(timezone, 4);

    let first_range_primary = first_range.primary;
    let first_range_secondary = first_range.secondary.unwrap();
    assert_eq!(first_range_primary.start().num_hours(), 12);
    assert_eq!(first_range_primary.end().num_hours(), 14);
    assert_eq!(first_range_secondary.start().num_hours(), -12);
    assert_eq!(first_range_secondary.end().num_hours(), -10);
    let second_range_primary = second_range.primary;
    let second_range_secondary = second_range.secondary.unwrap();
    assert_eq!(second_range_primary.start().num_hours(), 11);
    assert_eq!(second_range_primary.end().num_hours(), 14);
    assert_eq!(second_range_secondary.start().num_hours(), -12);
    assert_eq!(second_range_secondary.end().num_hours(), -9);
    let third_range_primary = third_range.primary;
    let third_range_secondary = third_range.secondary.unwrap();
    assert_eq!(third_range_primary.start().num_hours(), 10);
    assert_eq!(third_range_primary.end().num_hours(), 14);
    assert_eq!(third_range_secondary.start().num_hours(), -12);
    assert_eq!(third_range_secondary.end().num_hours(), -8);
}

#[test]
fn negative_offset_range_with_different_main_range() {
    let timezone = Duration::hours(-12);
    let first_range = UtcOffsetRange::new(timezone, 2);
    let second_range = UtcOffsetRange::new(timezone, 3);
    let third_range = UtcOffsetRange::new(timezone, 4);

    let first_range_primary = first_range.primary;
    let first_range_secondary = first_range.secondary.unwrap();
    assert_eq!(first_range_primary.start().num_hours(), -12);
    assert_eq!(first_range_primary.end().num_hours(), -10);
    assert_eq!(first_range_secondary.start().num_hours(), 12);
    assert_eq!(first_range_secondary.end().num_hours(), 14);

    let second_range_primary = second_range.primary;
    let second_range_secondary = second_range.secondary.unwrap();
    assert_eq!(second_range_primary.start().num_hours(), -12);
    assert_eq!(second_range_primary.end().num_hours(), -9);
    assert_eq!(second_range_secondary.start().num_hours(), 11);
    assert_eq!(second_range_secondary.end().num_hours(), 14);

    let third_range_primary = third_range.primary;
    let third_range_secondary = third_range.secondary.unwrap();
    assert_eq!(third_range_primary.start().num_hours(), -12);
    assert_eq!(third_range_primary.end().num_hours(), -8);
    assert_eq!(third_range_secondary.start().num_hours(), 10);
    assert_eq!(third_range_secondary.end().num_hours(), 14);
}
