pub fn is_leap_year(year: u64) -> bool {
    // define constants used to determine leap year
    const LEAP_YEAR_GAP: u64 = 4;
    const LEAP_YEAR_EXCEPT_GAP: u64 = 100;
    const LEAP_CYCLE_LEN: u64 = 400;

    // cycle through all options from most to least specific
    if (year % LEAP_CYCLE_LEN) == 0 {
        true
    } else if (year % LEAP_YEAR_EXCEPT_GAP) == 0 {
        false
    } else {
        (year % LEAP_YEAR_GAP) == 0
    }
}
