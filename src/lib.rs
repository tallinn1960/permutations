pub fn permute<T>(data: &[T]) -> impl Iterator<Item = Vec<&T>> {
    Permutator::new(data)
}

pub fn permute_cloned<T: Clone>(
    data: &[T],
) -> impl Iterator<Item = Vec<T>> + '_ {
    PermutatorCloned::new(data)
}

struct Permutator<'a, T> {
    data: &'a [T],
    indices: Vec<usize>,
    done: bool,
}

impl<'a, T> Permutator<'a, T> {
    fn new(data: &'a [T]) -> Self {
        let len = data.len();
        Self {
            data,
            indices: (0..len).collect(),
            done: false,
        }
    }
}

impl<'a, T> Iterator for Permutator<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result = self.indices.iter().map(|&i| &self.data[i]).collect();
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

struct PermutatorCloned<'a, T: Clone> {
    permutator: Permutator<'a, T>,
}

impl<'a, T: Clone> PermutatorCloned<'a, T> {
    fn new(data: &'a [T]) -> Self {
        Self {
            permutator: Permutator::new(data),
        }
    }
}

impl<'a, T: Clone> Iterator for PermutatorCloned<'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.permutator
            .next()
            .map(|v| v.into_iter().cloned().collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = vec![1, 2, 3];
        let perms = permute_cloned(&v).collect::<Vec<_>>();
        assert_eq!(perms.len(), 6);
        assert_eq!(perms[0], vec![1, 2, 3]);
        assert_eq!(perms[1], vec![1, 3, 2]);
        assert_eq!(perms[2], vec![2, 1, 3]);
        assert_eq!(perms[3], vec![2, 3, 1]);
        assert_eq!(perms[4], vec![3, 1, 2]);
        assert_eq!(perms[5], vec![3, 2, 1]);
    }
}
