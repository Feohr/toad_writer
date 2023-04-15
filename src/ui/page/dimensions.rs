// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

use std::default::Default;

#[derive(Debug)]
pub struct Pixels {
    height: i32,
    width: i32,
}

const NAS_DIMENSIONS: [(f64, f64); 4] = [
    // Letter
    (8.5_f64, 11_f64),
    // Legal
    (8.5_f64, 14_f64),
    // Tabloid
    (11_f64, 17_f64),
    // Ledger
    (17_f64, 11_f64),
];
const ISO_DIMENSIONS: [(f64, f64); 9] = [
    // A0
    (33.1_f64, 46.8_f64),
    // A1
    (23.4_f64, 33.1_f64),
    // A2
    (16.5_f64, 23.4_f64),
    // A3
    (11.7_f64, 16.5_f64),
    // A4
    (8.3_f64, 11.7_f64),
    // A5
    (5.8_f64, 8.5_f64),
    // A6
    (4.1_f64, 5.8_f64),
    // A7
    (2.9_f64, 4.1_f64),
    // A8
    (2.0_f64, 2.9_f64),
];

// North American Standard paper sizes
#[allow(unused)]
pub enum NASDimensions {
    Letter,
    Legal,
    Tabloid,
    Ledger,
}
// International ISO A-Series paper sizes
#[allow(unused)]
pub enum ISODimensions {
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
}

/*▇▇▇▇▇▇▇▇w▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

impl Default for NASDimensions {
    fn default() -> Self {
        Self::Letter
    }
}

impl Default for ISODimensions {
    fn default() -> Self {
        Self::A6
    }
}

impl Default for Pixels {
    fn default() -> Self {
        ISODimensions::default().get().with_resolution()
    }
}

impl ISODimensions {
    pub fn get(&self) -> Pixels {
        let (height, width) = match self {
            ISODimensions::A0 => ISO_DIMENSIONS[0],
            ISODimensions::A1 => ISO_DIMENSIONS[1],
            ISODimensions::A2 => ISO_DIMENSIONS[2],
            ISODimensions::A3 => ISO_DIMENSIONS[3],
            ISODimensions::A4 => ISO_DIMENSIONS[4],
            ISODimensions::A5 => ISO_DIMENSIONS[5],
            ISODimensions::A6 => ISO_DIMENSIONS[6],
            ISODimensions::A7 => ISO_DIMENSIONS[7],
            ISODimensions::A8 => ISO_DIMENSIONS[8],
        };

        Pixels {
            height: height as i32,
            width: width as i32,
        }
    }
}

impl NASDimensions {
    #[allow(unused)]
    pub fn get(&self) -> Pixels {
        let (height, width) = match self {
            NASDimensions::Letter => NAS_DIMENSIONS[0],
            NASDimensions::Legal => NAS_DIMENSIONS[1],
            NASDimensions::Tabloid => NAS_DIMENSIONS[2],
            NASDimensions::Ledger => NAS_DIMENSIONS[3],
        };

        Pixels {
            height: height as i32,
            width: width as i32,
        }
    }
}

impl Pixels {
    fn with_resolution(self) -> Self {
        let resolution = gtk::PrintSettings::new().resolution();
        Pixels {
            height: self.height * resolution,
            width: self.width * resolution,
        }
    }
}
