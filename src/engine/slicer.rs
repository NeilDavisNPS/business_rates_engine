use chrono::{Datelike, NaiveDate, Duration};
use crate::models::calculation::{DateRange, SegmentPeriod, OccupancyPeriodInput, QualifyingReliefFlags, PartOccupiedApportionment, RvEntry};
use crate::models::enums::OccupancyType;

#[derive(Debug, Clone)]
pub struct BillingSegmentWindow {
    pub financial_year: String,
    pub period: SegmentPeriod,
    pub year_days: i64,
}

#[derive(Debug, Clone)]
pub struct SlicedPeriodWindow {
    pub financial_year: String,
    pub period: SegmentPeriod,
    pub year_days: i64,
    pub occupancy_type: OccupancyType,
    pub period_relief_flags: QualifyingReliefFlags,
    pub part_apportionment: Option<PartOccupiedApportionment>,
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

pub fn slice_occupation_period(range: &DateRange) -> Vec<BillingSegmentWindow> {
    let mut segments = Vec::new();
    let mut current_start = range.start_date;

    while current_start <= range.end_date {
        let start_year = current_start.year();
        let fy_start_year = if current_start.month() >= 4 { start_year } else { start_year - 1 };
        let fy_end_date = NaiveDate::from_ymd_opt(fy_start_year + 1, 3, 31).unwrap();

        let current_end = if range.end_date < fy_end_date { range.end_date } else { fy_end_date };
        let days = (current_end - current_start).num_days() + 1;
        let year_days = if is_leap_year(fy_start_year + 1) { 366 } else { 365 };

        segments.push(BillingSegmentWindow {
            financial_year: format!("{}/{:02}", fy_start_year, (fy_start_year + 1) % 100),
            period: SegmentPeriod {
                start_date: current_start,
                end_date: current_end,
                days,
            },
            year_days,
        });

        current_start = current_end + Duration::days(1);
    }
    segments
}

pub fn slice_occupancy_periods(
    periods: &[OccupancyPeriodInput],
    global_flags: &QualifyingReliefFlags,
    rv_history: &[RvEntry],
) -> Vec<SlicedPeriodWindow> {
    let mut result = Vec::new();
    for p in periods {
        let range = DateRange {
            start_date: p.start_date,
            end_date: p.end_date,
        };
        let fy_windows = slice_occupation_period(&range);
        let flags = p.qualifying_relief_flags.clone().unwrap_or_else(|| global_flags.clone());
        for win in fy_windows {
            // Find any mid-segment rateable value effective dates within (win.period.start_date, win.period.end_date]
            let mut cut_dates = Vec::new();
            for entry in rv_history {
                if entry.effective_from > win.period.start_date && entry.effective_from <= win.period.end_date {
                    cut_dates.push(entry.effective_from);
                }
                if let Some(end) = entry.effective_to {
                    let next_day = end + Duration::days(1);
                    if next_day > win.period.start_date && next_day <= win.period.end_date {
                        cut_dates.push(next_day);
                    }
                }
            }
            cut_dates.sort();
            cut_dates.dedup();

            if cut_dates.is_empty() {
                result.push(SlicedPeriodWindow {
                    financial_year: win.financial_year,
                    period: win.period,
                    year_days: win.year_days,
                    occupancy_type: p.occupancy_type,
                    period_relief_flags: flags.clone(),
                    part_apportionment: p.part_occupied_apportionment.clone(),
                });
            } else {
                let mut cur_start = win.period.start_date;
                for cut in cut_dates {
                    let sub_end = cut - Duration::days(1);
                    let sub_days = (sub_end - cur_start).num_days() + 1;
                    result.push(SlicedPeriodWindow {
                        financial_year: win.financial_year.clone(),
                        period: SegmentPeriod {
                            start_date: cur_start,
                            end_date: sub_end,
                            days: sub_days,
                        },
                        year_days: win.year_days,
                        occupancy_type: p.occupancy_type,
                        period_relief_flags: flags.clone(),
                        part_apportionment: p.part_occupied_apportionment.clone(),
                    });
                    cur_start = cut;
                }
                let sub_days = (win.period.end_date - cur_start).num_days() + 1;
                result.push(SlicedPeriodWindow {
                    financial_year: win.financial_year,
                    period: SegmentPeriod {
                        start_date: cur_start,
                        end_date: win.period.end_date,
                        days: sub_days,
                    },
                    year_days: win.year_days,
                    occupancy_type: p.occupancy_type,
                    period_relief_flags: flags.clone(),
                    part_apportionment: p.part_occupied_apportionment.clone(),
                });
            }
        }
    }
    result.sort_by_key(|w| w.period.start_date);
    result
}
