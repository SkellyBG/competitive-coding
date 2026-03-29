use std::collections::{BTreeMap, BTreeSet, HashMap};

struct EventManager {
    priority_to_event_ids: BTreeMap<i32, BTreeSet<i32>>,
    event_id_to_priority: HashMap<i32, i32>,
}

impl EventManager {
    fn new(events: Vec<Vec<i32>>) -> Self {
        let mut event_id_to_priority = HashMap::new();
        let mut priority_to_event_ids: BTreeMap<i32, BTreeSet<i32>> = BTreeMap::new();
        for event in events {
            let event_id = event[0];
            let priority = event[1];
            event_id_to_priority.insert(event_id, priority);
            priority_to_event_ids
                .entry(priority)
                .or_default()
                .insert(event_id);
        }

        Self {
            priority_to_event_ids,
            event_id_to_priority,
        }
    }

    fn update_priority(&mut self, event_id: i32, new_priority: i32) {
        let old_priority = self.event_id_to_priority.get(&event_id);
        if let Some(old_priority) = old_priority {
            self.priority_to_event_ids
                .entry(*old_priority)
                .or_default()
                .remove(&event_id);
            self.priority_to_event_ids
                .entry(new_priority)
                .or_default()
                .insert(event_id);
            self.event_id_to_priority.insert(event_id, new_priority);
        }
    }

    fn poll_highest(&mut self) -> i32 {
        if let Some(mut entry) = self.priority_to_event_ids.last_entry() {
            let value = entry.get_mut();
            if value.len() == 0 {
                entry.remove();
                self.poll_highest()
            } else {
                let event_id = value.pop_first().unwrap();
                event_id
            }
        } else {
            -1
        }
    }
}
