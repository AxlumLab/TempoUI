use std::collections::BTreeMap;

/// Widgets keep their own view state. This keeps each destination's detail history.
#[derive(Debug, Default)]
pub struct RouteStacks<T> {
    stacks: BTreeMap<usize, Vec<T>>,
}

impl<T> RouteStacks<T> {
    pub fn top(&self, destination: usize) -> Option<&T> {
        self.stacks.get(&destination)?.last()
    }

    pub fn depth(&self, destination: usize) -> usize {
        self.stacks.get(&destination).map_or(0, Vec::len)
    }

    pub fn open(&mut self, destination: usize, route: T, replace: bool) {
        let stack = self.stacks.entry(destination).or_default();
        if replace {
            stack.pop();
        }
        stack.push(route);
    }

    pub fn back(&mut self, destination: usize) -> bool {
        self.stacks
            .get_mut(&destination)
            .and_then(Vec::pop)
            .is_some()
    }

    pub fn close(&mut self, destination: usize) {
        self.stacks.remove(&destination);
    }
}

#[cfg(test)]
mod tests {
    use super::RouteStacks;

    #[test]
    fn independent_destination_stacks_survive_switches() {
        let mut routes = RouteStacks::default();
        routes.open(0, "album", false);
        routes.open(1, "settings", false);
        routes.open(0, "song", false);
        assert_eq!(routes.top(0), Some(&"song"));
        assert!(routes.back(0));
        assert_eq!(routes.top(0), Some(&"album"));
        assert_eq!(routes.top(1), Some(&"settings"));
        routes.close(0);
        assert!(!routes.back(0));
        assert_eq!(routes.depth(1), 1);
    }

    #[test]
    fn replace_does_not_add_a_back_entry() {
        let mut routes = RouteStacks::default();
        routes.open(0, 1, true);
        routes.open(0, 2, true);
        assert_eq!(routes.top(0), Some(&2));
        assert_eq!(routes.depth(0), 1);
    }
}
