// Copyright 2017 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Render target specification.

// TODO: Really tighten up the terminology here.

/// A depth value, specifying which plane to select out of a 3D texture.
pub type Layer = u16;
/// Mipmap level to select in a texture.
pub type Level = u8;
/// A single depth value from a depth buffer.
pub type Depth = f32;
/// A single value from a stencil stencstencil buffer.
pub type Stencil = u8;

/// A screen space rectangle
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

/// A color with floating-point components.
pub type ColorValue = [f32; 4];

bitflags!(
    /// Mirroring flags, used for blitting
    #[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
    pub struct Mirror: u8 {
        ///
        const X  = 0x01;
        ///
        const Y  = 0x02;
    }
);
