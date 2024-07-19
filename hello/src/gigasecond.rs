use::time::PrimitiveDateTime as DateTime;
use time::ext::NumericalDuration;

pub fn after(start: DateTime) -> DateTime {
    start.checked_add(1000000000.seconds()).unwrap()
}