use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec2D<T> {
    data: Vec<T>,
    l1: usize,
    l2: usize,
}

impl<T> Vec2D<T> {
    #[inline]
    pub fn new(l1: usize, l2: usize, initial: T) -> Vec2D<T>
    where
        T: Copy,
    {
        Vec2D {
            data: vec![initial; l1 * l2],
            l1,
            l2,
        }
    }

    #[inline]
    fn get_index(&self, index: (usize, usize)) -> usize {
        let i1 = index.0;
        let i2 = index.1;
        if i1 >= self.l1 || i2 >= self.l2 {
            panic!("Out of bounds");
        }
        i2 + i1 * self.l2
    }

    #[inline]
    pub fn iter(&self) -> Vec2DIterator<'_, T> {
        Vec2DIterator {
            idx: 0,
            vec2d: self,
        }
    }
}

impl<T> Index<(usize, usize)> for Vec2D<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.data.index(self.get_index(index))
    }
}

impl<T> IndexMut<(usize, usize)> for Vec2D<T> {
    #[inline]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.data.index_mut(self.get_index(index))
    }
}

pub struct Vec2DIterator<'a, T> {
    idx: usize,
    vec2d: &'a Vec2D<T>,
}

impl<T> Iterator for Vec2DIterator<'_, T>
where
    T: Copy,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.vec2d.l1 * self.vec2d.l2 {
            None
        } else {
            let val = Some(self.vec2d.data[self.idx]);
            self.idx += 1;
            val
        }
    }
}
