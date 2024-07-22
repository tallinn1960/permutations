use core::fmt::Debug;

// by quinedot, Rust community forum
pub fn permute<'a, U, T, F>(
    data: &'a [T],
    accessor: F,
) -> Permutator<'a, T, F, U>
where
    F: FnMut(&'a T) -> U + 'a,
{
    Permutator::new(data, accessor)
}

#[derive(Clone)]
pub struct Permutator<'a, T, F, U>
where
    F: FnMut(&'a T) -> U + 'a,
{
    data: &'a [T],
    indices: Vec<usize>,
    done: bool,
    accessor: F,
}

impl<'a, T: Debug, F, U> Debug for Permutator<'a, T, F, U>
where
    F: FnMut(&'a T) -> U + 'a,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Permutator")
            .field("data", &self.data)
            .field("indices", &self.indices)
            .field("done", &self.done)
            .finish()
    }
}

impl<'a, T, F, U> Permutator<'a, T, F, U>
where
    F: FnMut(&'a T) -> U + 'a,
{
    fn new(data: &'a [T], accessor: F) -> Self {
        let len = data.len();
        Self {
            data,
            indices: (0..len).collect(),
            done: false,
            accessor,
        }
    }
}

impl<'a, T, F: FnMut(&'a T) -> U, U> Iterator for Permutator<'a, T, F, U> {
    type Item = Vec<U>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result = self
            .indices
            .iter()
            .copied()
            .map(|idx| (self.accessor)(&self.data[idx]))
            .collect();

        let mut i = self.indices.len() - 1;
        while i > 0 && self.indices[i - 1] >= self.indices[i] {
            i -= 1;
        }

        if i == 0 {
            self.done = true;
        } else {
            let mut j = self.indices.len() - 1;
            while self.indices[j] <= self.indices[i - 1] {
                j -= 1;
            }

            self.indices.swap(j, i - 1);
            self.indices[i..].reverse();
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute() {
        let v = vec!["1", "2", "3"];
        let perms = permute(&v, std::convert::identity).collect::<Vec<_>>();
        assert_eq!(perms.len(), 6);
        assert_eq!(perms[0], vec![&"1", &"2", &"3"]);
        assert_eq!(perms[1], vec![&"1", &"3", &"2"]);
        assert_eq!(perms[2], vec![&"2", &"1", &"3"]);
        assert_eq!(perms[3], vec![&"2", &"3", &"1"]);
        assert_eq!(perms[4], vec![&"3", &"1", &"2"]);
        assert_eq!(perms[5], vec![&"3", &"2", &"1"]);
    }

    #[test]
    fn test_permute_cloned() {
        let v = vec![1, 2, 3];
        let perms = permute(&v, Clone::clone).collect::<Vec<_>>();
        assert_eq!(perms.len(), 6);
        assert_eq!(perms[0], vec![1, 2, 3]);
        assert_eq!(perms[1], vec![1, 3, 2]);
        assert_eq!(perms[2], vec![2, 1, 3]);
        assert_eq!(perms[3], vec![2, 3, 1]);
        assert_eq!(perms[4], vec![3, 1, 2]);
        assert_eq!(perms[5], vec![3, 2, 1]);
    }

    #[test]
    fn test_traits() {
        let v = permute(&[1, 2, 3], std::convert::identity);
        let _ = v.clone();
        let _ = format!("{:?}", v);
    }
}
