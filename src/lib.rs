use chrono::Duration;
use std::ops::Sub;

pub trait Humanized {
    fn hhmmss(&self) -> String;

    fn hhmmssxxx(&self) -> String;
}

impl Humanized for Duration {
    fn hhmmss(&self) -> String {
        dbg!(self.num_hours());
        dbg!(self.num_minutes());
        dbg!(self.num_seconds());
        dbg!(self.num_milliseconds());

        if self.num_days() != 0 {
            return format!(
                "{:02}d {:02}:{:02}:{:02}",
                self.num_days(),
                self.sub(Duration::days(self.num_days())).num_hours(),
                self.sub(Duration::hours(self.num_hours())).num_minutes(),
                self.sub(Duration::minutes(self.num_minutes()))
                    .num_seconds()
            );
        }

        return format!(
            "{:02}:{:02}:{:02}",
            self.num_hours(),
            self.sub(Duration::hours(self.num_hours())).num_minutes(),
            self.sub(Duration::minutes(self.num_minutes()))
                .num_seconds()
        );
    }

    fn hhmmssxxx(&self) -> String {
        dbg!(self.num_hours());
        dbg!(self.num_minutes());
        dbg!(self.num_seconds());
        dbg!(self.num_milliseconds());

        if self.num_days() != 0 {
            return format!(
                "{:02}d {:02}:{:02}:{:02}.{:03}",
                self.num_days(),
                self.sub(Duration::days(self.num_days())).num_hours(),
                self.sub(Duration::hours(self.num_hours())).num_minutes(),
                self.sub(Duration::minutes(self.num_minutes()))
                    .num_seconds(),
                self.sub(Duration::seconds(self.num_seconds()))
                    .num_milliseconds()
            );
        }

        return format!(
            "{:02}:{:02}:{:02}.{:03}",
            self.num_hours(),
            self.sub(Duration::hours(self.num_hours())).num_minutes(),
            self.sub(Duration::minutes(self.num_minutes()))
                .num_seconds(),
            self.sub(Duration::seconds(self.num_seconds()))
                .num_milliseconds()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn days() {
        let duration: Duration = Duration::days(10);
        assert_eq!("10d 00:00:00", duration.hhmmss())
    }

    #[test]
    fn hours() {
        let duration: Duration =
            Duration::days(0) + Duration::hours(11) + Duration::minutes(12) + Duration::seconds(13);
        assert_eq!("11:12:13", duration.hhmmss())
    }

    #[test]
    fn days_with_hours() {
        let duration: Duration = Duration::days(10)
            + Duration::hours(11)
            + Duration::minutes(12)
            + Duration::seconds(13);
        assert_eq!("10d 11:12:13", duration.hhmmss())
    }

    #[test]
    fn daysfff() {
        let duration: Duration = Duration::days(10);
        assert_eq!("10d 00:00:00.000", duration.hhmmssxxx())
    }

    #[test]
    fn hoursfff() {
        let duration: Duration = Duration::days(0)
            + Duration::hours(11)
            + Duration::minutes(12)
            + Duration::seconds(13)
            + Duration::milliseconds(14);
        assert_eq!("11:12:13.014", duration.hhmmssxxx())
    }

    #[test]
    fn days_with_hoursfff() {
        let duration: Duration = Duration::days(10)
            + Duration::hours(11)
            + Duration::minutes(12)
            + Duration::seconds(13)
            + Duration::milliseconds(14);
        assert_eq!("10d 11:12:13.014", duration.hhmmssxxx())
    }
}
