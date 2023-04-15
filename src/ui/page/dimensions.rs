// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.
// SPDX-License-Identifier: BSD-3-Clause
//
// Redistribution and use in source and binary forms, with or without modification, are permitted
// provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of
// conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of
// conditions and the following disclaimer in the documentation and/or other materials provided
// with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to
// endorse or promote products derived from this software without specific prior written
// permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR
// IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND
// FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
// OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

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
