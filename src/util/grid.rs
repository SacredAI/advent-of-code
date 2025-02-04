use crate::util::position::*;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub width: i32,
    pub height: i32,
    pub grid: Vec<T>,
}

pub struct Positions<'a, T> {
    pub x: i32,
    pub y: i32,
    pub grid: &'a Grid<T>,
}

impl<'a, T> Iterator for Positions<'a, T> {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.grid.height {
            return None;
        }
        let pos = Pos::new(self.x, self.y);
        self.x += 1;
        if self.x == self.grid.width {
            self.x = 0;
            self.y += 1;
        }
        Some(pos)
    }
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Self {
        let raw: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
        let width = raw[0].len() as i32;
        let height = raw.len() as i32;
        let mut grid = Vec::with_capacity((width * height) as usize);
        raw.iter().for_each(|slice| grid.extend_from_slice(slice));
        Grid {
            width,
            height,
            grid,
        }
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let point = Pos::new(x, y);
                print!("{}", self[point] as char);
            }
            println!();
        }
        println!();
    }

    pub fn positions(&self) -> Positions<u8> {
        Positions {
            x: 0,
            y: 0,
            grid: self,
        }
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    #[inline]
    pub fn find(&self, needle: T) -> Option<Pos> {
        let to_point = |index| {
            let x = (index as i32) % self.width;
            let y = (index as i32) / self.width;
            Pos::new(x, y)
        };
        self.grid.iter().position(|&h| h == needle).map(to_point)
    }
}

impl<T: Copy> Grid<T> {
    pub fn new(width: i32, height: i32, value: T) -> Grid<T> {
        Grid {
            width,
            height,
            grid: vec![value; (width * height) as usize],
        }
    }
}

impl<T> Grid<T> {
    #[inline]
    pub fn same_size_with<U: Copy>(&self, value: U) -> Grid<U> {
        Grid {
            width: self.width,
            height: self.height,
            grid: vec![value; (self.width * self.height) as usize],
        }
    }

    #[inline]
    pub fn contains(&self, point: Pos) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }
}

impl<T> Index<Pos> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Pos) -> &Self::Output {
        &self.grid[(self.width * index.y + index.x) as usize]
    }
}

impl<T> IndexMut<Pos> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        &mut self.grid[(self.width * index.y + index.x) as usize]
    }
}
