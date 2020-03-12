/*
 * Copyright © 2020, G.Ralph Kuntz, MD.
 *
 * Licensed under the Apache License, Version 2.0(the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIC
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#[macro_export]

macro_rules! op_other {
    ($name:ident, $self_:ident, $other:ident, $body:expr) => {
        pub fn $name(&mut $self_, $other: &Cell) -> bool {
            let prev = $self_.0;
            $body;
            // assert_ne!($self_.0, 0, "empty cell: was: {:b}, other: {:b}, now: {:b}", prev, $other.0, $self_.0);
            $self_.0 != prev
        }
    };
}

use super::ALL_DIGITS;
use std::ops::BitOr;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Cell(pub u16);

lazy_static! {
    pub static ref BIT_COUNT: [u8; 1024] = {
        let mut bits: [u8; 1024] = [0; 1024];
        for i in 0..1024 {
            // Use Brian Kernighan's algorithm to count bits set to 1.
            let mut n = i;
            let mut count = 0;
            while n != 0 {
                n &= n - 1;
                count += 1;
            }
            bits[i] = count;
        }
        bits
    };
}

impl Cell {
    pub fn count(&self) -> usize {
        BIT_COUNT[usize::from(self.0)] as usize
    }

    pub fn digits(&self) -> Vec<usize> {
        let mut d = Vec::new();
        for i in ALL_DIGITS {
            if self.0 & (1 << i) != 0 {
                d.push(i);
            }
        }
        d
    }

    op_other!(and, self, other, self.0 &= other.0);

    op_other!(and_not, self, other, self.0 &= !other.0);

    op_other!(replace, self, other, self.0 = other.0);
}

impl BitOr for Cell {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Cell(self.0 | rhs.0)
    }
}

impl ToString for Cell {
    fn to_string(&self) -> String {
        let mut d = Vec::<String>::new();
        for i in ALL_DIGITS {
            if self.0 & (1 << i) != 0 {
                d.push(i.to_string());
            }
        }
        d.join(", ")
    }
}
