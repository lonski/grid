//! # Grid
//!
//! Helper library for manipulating 2d grid.

use std::iter::FromIterator;
use std::fmt;

#[derive(Clone)]
pub struct Grid {
    tiles: Vec<char>,
    width: usize,
}

impl Grid {
    /// Creates new grid filled with specified character
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::Grid;
    ///
    /// let grid = Grid::filled_with(2,2,'x');
    ///
    /// assert_eq!(&grid.to_string(), "xx\nxx\n");
    /// ```
    pub fn filled_with(width: usize, height: usize, fill: char) -> Self {
        Grid {
            tiles: (0..width * height).map(|_| fill).collect(),
            width: width,
        }
    }

    /// Creates new grid of defined dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::Grid;
    ///
    /// let grid = Grid::new(2,3);
    ///
    /// assert_eq!(grid.width(), 2);
    /// assert_eq!(grid.height(), 3);
    /// ```
    pub fn new(width: usize, height: usize) -> Self {
        Grid::filled_with(width, height, '#')
    }

    /// Creates grid from string.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::Grid;
    ///
    /// let grid = Grid::from("#*#\n| |\n+-+");
    ///
    /// assert_eq!(grid.width(), 3);
    /// assert_eq!(grid.to_string(), "#*#\n| |\n+-+\n");
    ///
    /// ```
    pub fn from(tiles: &str) -> Self {
        Grid {
            tiles: tiles
                .lines()
                .map(|l| l.chars().collect::<Vec<char>>())
                .fold(Vec::new(), |mut acc, v| {
                    acc.extend(&v);
                    acc
                }),
            width: tiles.lines().next().unwrap().len(),
        }
    }

    /// Returns tile character. When specified coordinates are out of grid bound None is returned.
    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        match self.cord_to_pos(x, y) {
            Some(pos) => Some(self.tiles[pos]),
            None => None,
        }
    }

    /// Sets tile character on specified coordinates.
    /// Returns true if character was set.
    /// Returns false when tile was not set (specified coordinates were out of grid bounds).
    pub fn set(&mut self, x: usize, y: usize, new_tile: char) -> bool {
        match self.cord_to_pos(x, y) {
            Some(pos) => {
                self.tiles[pos] = new_tile;
                true
            }
            None => false,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.tiles.len() / self.width
    }

    /// Fills grid with given character. Works like a flood fill replacing all neighbour tile's
    /// characters which are differend than specified fill character.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::Grid;
    ///
    /// // ########
    /// // #......#
    /// // #.####.#
    /// // #.####.#
    /// // #......#
    /// // ########
    /// let mut grid = Grid::from("########\n#......#\n#.####.#\n#.####.#\n#......#\n########");
    ///
    /// grid.fill(2,2,'.');
    ///
    /// // ########
    /// // #......#
    /// // #......#
    /// // #......#
    /// // #......#
    /// // ########
    /// assert_eq!(&grid.to_string(),"########\n#......#\n#......#\n#......#\n#......#\n########\n");
    /// ```
    ///
    /// ```
    /// use grid::Grid;
    ///
    /// // #..#
    /// // ####
    /// let mut grid = Grid::from("#..#\n####");
    /// grid.fill(1,0,'+');
    ///
    /// assert_eq!(grid.to_string(), "++++\n++++\n");
    /// ```
    pub fn fill(&mut self, start_x: usize, start_y: usize, fill: char) {
        let mut frontier = vec![(start_x, start_y)];
        while !frontier.is_empty() {
            let (cx, cy) = frontier.pop().unwrap();
            self.set(cx, cy, fill);
            for (nx, ny) in self.neighbours(cx, cy) {
                if let Some(c) = self.get(nx, ny) {
                    if c != fill {
                        frontier.push((nx, ny));
                    }
                }
            }
        }
    }

    /// Counts tiles with specified character.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::Grid;
    ///
    /// let grid = Grid::from("#.\n##");
    ///
    /// assert_eq!(grid.count('.'), 1);
    /// assert_eq!(grid.count('#'), 3);
    /// ```
    pub fn count(&self, tile: char) -> usize {
        self.tiles.iter().filter(|&c| *c == tile).count()
    }

    /// Returns coordinates of neighbour tiles (including diagonal).
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::Grid;
    ///
    /// let grid = Grid::new(10,10);
    ///
    /// assert_eq!(grid.neighbours(5,5), vec![(5, 4), (6, 5), (5, 6), (4, 5), (6, 6), (6, 4), (4, 4), (4, 6)]);
    /// assert_eq!(grid.neighbours(0,0), vec![(1, 0), (0, 1), (1, 1)]);
    /// assert_eq!(grid.neighbours(9,9), vec![(9, 8), (8, 9), (8, 8)]);
    /// assert_eq!(grid.neighbours(100,100), vec![]);
    /// assert_eq!(grid.neighbours(0,100), vec![]);
    /// assert_eq!(grid.neighbours(100,0), vec![]);
    /// ```
    pub fn neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut nb = Vec::new();
        let height = self.height();
        let width = self.width();
        if x > width || y > height {
            return nb;
        }
        if y > 0 && y < height {
            nb.push((x, (y as i32 - 1) as usize));
        }
        if x < width - 1 {
            nb.push((x + 1, y));
        }
        if y < height - 1 {
            nb.push((x, y + 1));
        }
        if x > 0 && x < width {
            nb.push(((x as i32 - 1) as usize, y));
        }
        if x < width - 1 && y < height - 1 {
            nb.push((x + 1, y + 1));
        }
        if x < width - 1 && y > 0 && y < height {
            nb.push((x + 1, (y as i32 - 1) as usize));
        }
        if x > 0 && y > 0 && x < width && y < height {
            nb.push((x - 1, y - 1));
        }
        if x > 0 && x < width && y < height - 1 {
            nb.push(((x as i32 - 1) as usize, y + 1));
        }
        nb
    }

    fn cord_to_pos(&self, x: usize, y: usize) -> Option<usize> {
        let pos = y * self.width + x;
        if pos < self.tiles.len() {
            Some(pos)
        } else {
            None
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let as_str: String = self.tiles
            .chunks(self.width)
            .map(|c| String::from_iter(c))
            .fold(String::new(), |acc, row| format!("{}{}\n", acc, row));
        write!(f, "{}", as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_grid() {
        assert_eq!(Grid::new(2, 3).to_string(), "##\n##\n##\n");
    }

    #[test]
    fn should_change_tile() {
        let mut grid = Grid::new(2, 3);
        grid.set(1, 2, 'x');
        assert_eq!(grid.to_string(), "##\n##\n#x\n");
        assert_eq!(grid.get(1, 2).unwrap(), 'x');
    }

    #[test]
    fn should_return_true_only_if_successfully_set_tile() {
        let mut grid = Grid::new(1, 1);
        assert!(!grid.set(1, 0, '@'));
        assert_eq!(grid.to_string(), "#\n");
        assert!(grid.set(0, 0, '@'));
        assert_eq!(grid.to_string(), "@\n");
    }

    #[test]
    fn should_return_correct_dimmensions() {
        let grid = Grid::new(5, 8);
        assert_eq!(grid.width(), 5);
        assert_eq!(grid.height(), 8);
    }
}
