use std::collections::{HashMap, VecDeque};

trait TraveralStore<S>: Extend<S> {
    fn next_state(&mut self) -> Option<S>;
}

impl<S> TraveralStore<S> for Vec<S> {
    fn next_state(&mut self) -> Option<S> {
        self.pop()
    }
}

impl<S> TraveralStore<S> for VecDeque<S> {
    fn next_state(&mut self) -> Option<S> {
        self.pop_front()
    }
}

#[inline]
fn traverser<S, F, G>(mut ts: impl TraveralStore<S>, mut next_states_fn: F, mut interrupt_fn: G)
where
    F: FnMut(S) -> Option<Vec<S>>,
    G: FnMut(&S) -> bool,
{
    while let Some(state) = ts.next_state() {
        if interrupt_fn(&state) {
            return;
        }

        if let Some(next_states) = next_states_fn(state) {
            ts.extend(next_states);
        }
    }
}

#[inline]
pub fn complete_bfs<S, F>(initial_states: impl Iterator<Item = S>, next_states_fn: F)
where
    F: FnMut(S) -> Option<Vec<S>>,
{
    interruptable_bfs(initial_states, next_states_fn, |_| false);
}

#[inline]
pub fn interruptable_bfs<S, F, G>(
    initial_states: impl Iterator<Item = S>,
    next_states_fn: F,
    terminal_state_fn: G,
) where
    F: FnMut(S) -> Option<Vec<S>>,
    G: FnMut(&S) -> bool,
{
    traverser(
        VecDeque::from_iter(initial_states),
        next_states_fn,
        terminal_state_fn,
    )
}

#[inline]
pub fn complete_dfs<S, F>(initial_states: impl Iterator<Item = S>, next_states_fn: F)
where
    F: FnMut(S) -> Option<Vec<S>>,
{
    interruptable_dfs(initial_states, next_states_fn, |_| false)
}

#[inline]
pub fn interruptable_dfs<S, F, G>(
    initial_states: impl Iterator<Item = S>,
    next_states_fn: F,
    terminal_state_fn: G,
) where
    F: FnMut(S) -> Option<Vec<S>>,
    G: FnMut(&S) -> bool,
{
    traverser(
        Vec::from_iter(initial_states),
        next_states_fn,
        terminal_state_fn,
    )
}

#[inline]
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

#[inline]
pub fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

pub fn print_graph<T: std::fmt::Debug>(graph: &HashMap<(i32, i32), T>) {
    for y in 0.. {
        for x in 0.. {
            let Some(ch) = graph.get(&(x, y)) else {
                if x == 0 {
                    return;
                } else {
                    break;
                }
            };
            print!("{:?}", ch);
        }
        print!("\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_bfs_init() {
        let mut invocations = 0;
        let initial_states = [0, 1, 2];
        complete_bfs(initial_states.iter(), |_| {
            invocations += 1;
            None
        });
        assert_eq!(invocations, 3);
    }

    #[test]
    fn basic_bfs_neighbors() {
        let mut invocations = 0;
        let initial_states = [0];
        complete_bfs(initial_states.into_iter(), |n| {
            assert_eq!(n, invocations);
            invocations += 1;
            if n < 5 {
                Some(vec![n + 1])
            } else {
                None
            }
        });
    }

    #[test]
    fn interrupted_bfs() {
        let mut invocations = 0;
        interruptable_bfs(
            [0].into_iter(),
            |n| {
                invocations += 1;
                Some(vec![n + 1])
            },
            |n| *n > 3,
        );
        assert_eq!(4, invocations);
    }
}
