pub mod overlapping_time {
    use std::cmp::{max, min};
    use std::collections::{BTreeSet, HashMap};

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord,Copy)]
    pub struct Meeting {
        start_time: i32,
        end_time: i32,
    }

    impl Meeting {
        pub fn new(start_time: i32, end_time: i32) -> Result<Self, &'static str> {
            if start_time > end_time {
                return Err("Invalid meeting time: start_time must be <= end_time");
            }
            Ok(Self {
                start_time,
                end_time,
            })
        }

        pub fn get_start_time(&self) -> i32 {
            self.start_time
        }

        pub fn get_end_time(&self) -> i32 {
            self.end_time
        }
    }

    pub struct Scheduler {
        meetings: HashMap<String, BTreeSet<Meeting>>,
    }

    impl Scheduler {
        pub fn new() -> Self {
            Self {
                meetings: HashMap::new(),
            }
        }

        pub fn has_meeting(&self, key: &String) -> bool {
            self.meetings.contains_key(key)
        }

        pub fn get_meetings(&self, key: String) -> Option<&BTreeSet<Meeting>> {
            self.meetings.get(&key)
        }

        pub fn add_meeting(&mut self, name: String, meeting: Meeting) {
            let new_start = meeting.get_start_time();
            let new_end = meeting.get_end_time();
            let mut merged_start = new_start;
            let mut merged_end = new_end;

            let meetings_set = self.meetings.entry(name.clone()).or_insert(BTreeSet::new());

            // Unwrap `Meeting::new()` before using it in range()
            let start_bound = Meeting::new(i32::MIN, i32::MIN).unwrap();
            let end_bound = Meeting::new(new_end, i32::MAX).unwrap();

            // Use references in `range()`
            let overlapping_meetings: Vec<Meeting> = meetings_set
                .range(&start_bound..=&end_bound)
                .cloned()
                .collect();

            // Compute new merged time range
            for m in &overlapping_meetings {
                merged_start = min(merged_start, m.get_start_time());
                merged_end = max(merged_end, m.get_end_time());
            }

            // Remove overlapping meetings using `retain()`
            meetings_set
                .retain(|m| !(m.get_end_time() >= new_start && m.get_start_time() <= new_end));

            // Insert the new merged meeting
            meetings_set.insert(Meeting::new(merged_start, merged_end).unwrap());
        }

        pub fn show_meetings(&self, name: String) {
            if !(self.has_meeting(&name)) {
                println!("No meetings for {}", name)
            } else {
                println!("Meetings for {} : ", name);
                for meeting in self.get_meetings(name.clone()).unwrap() {
                    println!("\t{:?}", meeting);
                }
            }
            println!("************");
        }
        pub fn get_overlaps(&self, name1: String, name2: String) -> Option<Vec<Meeting>> {
            if !(self.has_meeting(&name1) && self.has_meeting(&name2)) {
                None
            } else {
                let meetings1 = self.get_meetings(name1);
                let meetings2 = self.get_meetings(name2);

                if meetings1.is_none() || meetings2.is_none() {
                    return None;
                }

                let meetings1 = meetings1.unwrap();
                let meetings2 = meetings2.unwrap();

                let mut overlap = Vec::new();
                for meeting1 in meetings1 {
                    for meeting2 in meetings2 {
                        if max(meeting1.get_start_time(), meeting2.get_start_time())
                            < min(meeting1.get_end_time(), meeting2.get_end_time())
                        {
                            overlap.push(
                                Meeting::new(
                                    max(meeting1.get_start_time(), meeting2.get_start_time()),
                                    min(meeting1.get_end_time(), meeting2.get_end_time()),
                                )
                                .unwrap(),
                            );
                        }
                    }
                }
                Some(overlap)
            }
        }
    }
}
