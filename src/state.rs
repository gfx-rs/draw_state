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

//! Fixed-function hardware state.
//!
//! Configures the primitive assembly (PA), rasterizer, and output merger (OM) blocks.

use std::default::Default;
use std::fmt;

use target;

/// The front face winding order of a set of vertices.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub enum FrontFace {
    /// Clockwise winding order.
    Clockwise,
    /// Counter-clockwise winding order.
    CounterClockwise,
}

/// Width of a line.
/// Could be f32 if not for Hash deriving issues.
pub type LineWidth = i32;
/// Slope depth offset factor
/// Could be f32 if not for Hash deriving issues.
pub type OffsetSlope = i32;
/// Number of units to offset, where
/// the unit is the minimal difference in the depth value
/// dictated by the precision of the depth buffer.
pub type OffsetUnits = i32;

/// How to offset vertices in screen space, if at all.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Offset(pub OffsetSlope, pub OffsetUnits);

/// Which face, if any, to cull.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub enum CullFace {
    Nothing,
    Front,
    Back,
}

/// How to rasterize a primitive.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub enum RasterMethod {
    /// Rasterize as a point.
    Point,
    /// Rasterize as a line with the given width.
    Line(LineWidth),
    /// Rasterize as a face.
    Fill
}

/// Multi-sampling rasterization mode
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct MultiSample;
    //sample_mask: u16,
    //alpha_to_coverage: bool,

/// Primitive rasterization state. Note that GL allows different raster
/// method to be used for front and back, while this abstraction does not.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Rasterizer {
    /// Which vertex winding is considered to be the front face for culling.
    pub front_face: FrontFace,
    /// Which face should be culled.
    pub cull_face: CullFace,
    /// How to rasterize this primitive.
    pub method: RasterMethod,
    /// Any polygon offset to apply.
    pub offset: Option<Offset>,
    /// Multi-sampling mode.
    pub samples: Option<MultiSample>,
}

impl Rasterizer {
    /// Create a new filling rasterizer.
    pub fn new_fill() -> Self {
        Rasterizer {
            front_face: FrontFace::CounterClockwise,
            cull_face: CullFace::Nothing,
            method: RasterMethod::Fill,
            offset: None,
            samples: None,
        }
    }

    /// Add back face culling.
    pub fn with_cull_back(self) -> Self {
        Rasterizer {
            cull_face: CullFace::Back,
            ..self
        }
    }

    /// Add polygon offset.
    pub fn with_offset(self, slope: f32, units: OffsetUnits) -> Self {
        Rasterizer {
            offset: Some(Offset(slope as OffsetSlope, units)),
            ..self
        }
    }
}

/// A pixel-wise comparison function.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub enum Comparison {
    /// `false`
    Never,
    /// `x < y`
    Less,
    /// `x <= y`
    LessEqual,
    /// `x == y`
    Equal,
    /// `x >= y`
    GreaterEqual,
    /// `x > y`
    Greater,
    /// `x != y`
    NotEqual,
    /// `true`
    Always,
}

/// Stencil mask operation.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub enum StencilOp {
    /// Keep the current value in the stencil buffer (no change).
    Keep,
    /// Set the value in the stencil buffer to zero.
    Zero,
    /// Set the stencil buffer value to `value` from `StencilSide`
    Replace,
    /// Increment the stencil buffer value, clamping to its maximum value.
    IncrementClamp,
    /// Increment the stencil buffer value, wrapping around to 0 on overflow.
    IncrementWrap,
    /// Decrement the stencil buffer value, clamping to its minimum value.
    DecrementClamp,
    /// Decrement the stencil buffer value, wrapping around to the maximum value on overflow.
    DecrementWrap,
    /// Bitwise invert the current value in the stencil buffer.
    Invert,
}

/// Complete stencil state for a given side of a face.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct StencilSide {
    /// Comparison function to use to determine if the stencil test passes.
    pub fun: Comparison,
    /// A mask that is ANDd with both the stencil buffer value and the reference value when they
    /// are read before doing the stencil test.
    pub mask_read: target::Stencil,
    /// A mask that is ANDd with the stencil value before writing to the stencil buffer.
    pub mask_write: target::Stencil,
    /// What operation to do if the stencil test fails.
    pub op_fail: StencilOp,
    /// What operation to do if the stenil test passes but the depth test fails.
    pub op_depth_fail: StencilOp,
    /// What operation to do if both the depth and stencil test pass.
    pub op_pass: StencilOp,
}

impl Default for StencilSide {
    fn default() -> Self {
        StencilSide {
            fun: Comparison::Always,
            mask_read: target::Stencil::max_value(),
            mask_write: target::Stencil::max_value(),
            op_fail: StencilOp::Keep,
            op_depth_fail: StencilOp::Keep,
            op_pass: StencilOp::Keep,
        }
    }
}

/// Complete stencil state, specifying how to handle the front and back side of a face.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Stencil {
    pub front: StencilSide,
    pub back: StencilSide,
}

impl Stencil {
    /// Create a new stencil state with a given function.
    pub fn new(fun: Comparison, mask: target::Stencil,
               ops: (StencilOp, StencilOp, StencilOp))
               -> Self {
        let side = StencilSide {
            fun: fun,
            mask_read: mask,
            mask_write: mask,
            op_fail: ops.0,
            op_depth_fail: ops.1,
            op_pass: ops.2,
        };
        Stencil {
            front: side,
            back: side,
        }
    }
}

/// Depth test state.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Depth {
    /// Comparison function to use.
    pub fun: Comparison,
    /// Specify whether to write to the depth buffer or not.
    pub write: bool,
}

impl Default for Depth {
    fn default() -> Self {
        Depth {
            fun: Comparison::Always,
            write: false,
        }
    }
}

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub enum Equation {
    /// Adds source and destination.
    /// Source and destination are multiplied by blending parameters before addition.
    Add,
    /// Subtracts destination from source.
    /// Source and destination are multiplied by blending parameters before subtraction.
    Sub,
    /// Subtracts source from destination.
    /// Source and destination are multiplied by blending parameters before subtraction.
    RevSub,
    /// Component-wise minimum value of source and destination.
    /// Blending parameters are ignored.
    Min,
    /// Component-wise maximum value of source and destination.
    /// Blending parameters are ignored.
    Max,
}

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub enum BlendValue {
    SourceColor,
    SourceAlpha,
    DestColor,
    DestAlpha,
    ConstColor,
    ConstAlpha,
}

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub enum Factor {
    Zero,
    One,
    SourceAlphaSaturated,
    ZeroPlus(BlendValue),
    OneMinus(BlendValue),
}

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct BlendChannel {
    pub equation: Equation,
    pub source: Factor,
    pub destination: Factor,
}

impl Default for BlendChannel {
    fn default() -> Self {
        BlendChannel {
            equation: Equation::Add,
            source: Factor::One,
            destination: Factor::Zero,
        }
    }
}

#[allow(missing_docs)]
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Blend {
    pub color: BlendChannel,
    pub alpha: BlendChannel,
}

impl Blend {
    /// Create a new blend state with a given equation.
    pub fn new(eq: Equation, src: Factor, dst: Factor) -> Self {
        let chan = BlendChannel {
            equation: eq,
            source: src,
            destination: dst,
        };
        Blend {
            color: chan,
            alpha: chan,
        }
    }
}

impl fmt::Debug for Blend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Blend {{ color: {:?}, alpha: {:?}}}",
               self.color, self.alpha)
    }
}

bitflags!(
    #[allow(missing_docs)]
    #[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
    pub struct ColorMask: u8 {
        #[allow(missing_docs)]
        const RED     = 0x1;
        #[allow(missing_docs)]
        const GREEN   = 0x2;
        #[allow(missing_docs)]
        const BLUE    = 0x4;
        #[allow(missing_docs)]
        const ALPHA   = 0x8;
    }
);

/// The state of an active color render target
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Color {
    /// Color mask to use.
    pub mask: ColorMask,
    /// Optional blending.
    pub blend: Option<Blend>,
}

impl Default for Color {
    fn default() -> Self {
        Color {
            mask: ColorMask::all(),
            blend: None,
        }
    }
}

/// The complete set of the rasterizer reference values.
/// Switching these doesn't roll the hardware context.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct RefValues {
    /// Stencil front and back values.
    pub stencil: (target::Stencil, target::Stencil),
    /// Constant blend color.
    pub blend: target::ColorValue,
}

impl Default for RefValues {
    fn default() -> Self {
        RefValues {
            stencil: (0, 0),
            blend: [0f32; 4],
        }
    }
}
