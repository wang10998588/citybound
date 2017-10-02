pub const TICKS_PER_SIM_SECOND: usize = 1;
pub const TICKS_PER_SIM_MINUTE: usize = 60 * TICKS_PER_SIM_SECOND;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ticks(pub usize);

impl From<Duration> for Ticks {
    fn from(d_secs: Duration) -> Ticks {
        Ticks(d_secs.0 * TICKS_PER_SIM_SECOND)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration(pub usize);

impl Duration {
    pub fn as_seconds(&self) -> f32 {
        self.0 as f32
    }

    pub fn as_minutes(&self) -> f32 {
        self.0 as f32 / 60.0
    }

    pub fn as_hours(&self) -> f32 {
        self.as_minutes() / 60.0
    }
}

impl ::std::ops::Add for Duration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Duration(self.0 + rhs.0)
    }
}

impl ::std::ops::AddAssign for Duration {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant(usize);

impl Instant {
    pub fn new(ticks: usize) -> Self {
        Instant(ticks)
    }

    pub fn ticks(&self) -> usize {
        self.0
    }

    pub fn iticks(&self) -> isize {
        self.0 as isize
    }
}

impl<D: Into<Ticks>> ::std::ops::Add<D> for Instant {
    type Output = Self;

    fn add(self, rhs: D) -> Self {
        Instant(self.0 + rhs.into().0)
    }
}

impl<D: Into<Ticks>> ::std::ops::AddAssign<D> for Instant {
    fn add_assign(&mut self, rhs: D) {
        self.0 += rhs.into().0
    }
}

impl<D: Into<Ticks>> ::std::ops::Sub<D> for Instant {
    type Output = Self;

    fn sub(self, rhs: D) -> Self {
        Instant(self.0 - rhs.into().0)
    }
}

impl<D: Into<Ticks>> ::std::ops::SubAssign<D> for Instant {
    fn sub_assign(&mut self, rhs: D) {
        self.0 -= rhs.into().0
    }
}


#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct TimeOfDay {
    minutes_since_midnight: u16,
}

impl TimeOfDay {
    pub fn new(h: usize, m: usize) -> Self {
        TimeOfDay { minutes_since_midnight: m as u16 + (h * 60) as u16 }
    }

    pub fn from_instant(current_instant: Instant) -> Self {
        TimeOfDay {
            minutes_since_midnight: 7 * 60 +
                (current_instant.ticks() / TICKS_PER_SIM_MINUTE) as u16,
        }
    }

    pub fn hours_minutes(&self) -> (usize, usize) {
        (
            (self.minutes_since_midnight / 60) as usize,
            (self.minutes_since_midnight % 60) as usize,
        )
    }
}

impl<D: Into<Duration>> ::std::ops::Add<D> for TimeOfDay {
    type Output = Self;

    fn add(self, rhs: D) -> Self {
        TimeOfDay {
            minutes_since_midnight: self.minutes_since_midnight + (rhs.into().0 / 60) as u16,
        }
    }
}

impl<D: Into<Duration>> ::std::ops::AddAssign<D> for TimeOfDay {
    fn add_assign(&mut self, rhs: D) {
        self.minutes_since_midnight += (rhs.into().0 / 60) as u16
    }
}

impl<D: Into<Duration>> ::std::ops::Sub<D> for TimeOfDay {
    type Output = Self;

    fn sub(self, rhs: D) -> Self {
        TimeOfDay {
            minutes_since_midnight: self.minutes_since_midnight - (rhs.into().0 / 60) as u16,
        }
    }
}

impl<D: Into<Duration>> ::std::ops::SubAssign<D> for TimeOfDay {
    fn sub_assign(&mut self, rhs: D) {
        self.minutes_since_midnight -= (rhs.into().0 / 60) as u16
    }
}
