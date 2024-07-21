pub fn permute<'a, T>(data: &'a [T]) -> impl Iterator<Item = Vec<&T>> {
    Permutator::<'a, T, Uncloned>::new(data)
}

pub fn permute_cloned<'a, T: Clone>(
    data: &'a [T],
) -> impl Iterator<Item = Vec<T>> + 'a {
    Permutator::<'a, T, Cloned>::new(data)
}

trait GetValue<'a, T> {
    type Item;
    fn get_value(x: &'a T) -> Self::Item;
}

struct Uncloned;

impl<'a, T: 'a> GetValue<'a, T> for Uncloned {
    type Item = &'a T;
    fn get_value(x: &'a T) -> Self::Item {
        x
    }
}

struct Cloned;

impl<'a, T: 'a + Clone> GetValue<'a, T> for Cloned {
    type Item = T;
    fn get_value(x: &'a T) -> Self::Item {
        x.clone()
    }
}

struct Permutator<'a, T, GV: GetValue<'a, T>> {
    data: &'a [T],
    indices: Vec<usize>,
    done: bool,
    m: std::marker::PhantomData<GV>,
}

impl<'a, T, GV: GetValue<'a, T>> Permutator<'a, T, GV> {
    fn new(data: &'a [T]) -> Self {
        let len = data.len();
        Self {
            data,
            indices: (0..len).collect(),
            done: false,
            m: std::marker::PhantomData,
        }
    }
}

impl<'a, T, GV: GetValue<'a, T>> Iterator for Permutator<'a, T, GV> {
    type Item = Vec<GV::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result = self
            .indices
            .iter()
            .map(|&i| GV::get_value(&self.data[i]))
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
