// Copyright 2014 The Gfx-rs Developers.
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

//! Graphics state blocks for gfx-rs

#![deny(missing_docs, missing_copy_implementations)]

#[macro_use]
extern crate bitflags;

pub mod state;
pub mod target;

use state::{BlendValue, CullFace, Equation, FrontFace, RasterMethod, RefValues, StencilOp};
use target::{Mask, Rect, Stencil};

/// Compile-time maximum MRT count.
pub const MAX_COLOR_TARGETS:      usize = 4;

/// An assembly of states that affect regular draw calls.
/// Note: reference values are separated from their control blocks
/// due to the fact they can be provided separately from the state setup
/// on the modern hardware (DX11+).
#[must_use]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct DrawState {
    /// How to rasterize geometric primitives.
    pub primitive: state::Primitive,
    /// Multi-sampling mode
    pub multi_sample: Option<state::MultiSample>,
    /// Scissor mask to use. If set, no pixel outside of this rectangle (in screen space) will be
    /// written to as a result of rendering.
    pub scissor: Option<Rect>,
    /// Stencil test to use. If None, no stencil testing is done.
    pub stencil: Option<state::Stencil>,
    /// Depth test to use. If None, no depth testing is done.
    pub depth: Option<state::Depth>,
    /// Blend function to use. If None, no blending is done.
    pub blend: [Option<state::Blend>; MAX_COLOR_TARGETS],
    /// A set of reference values.
    pub ref_values: RefValues,
}

/// Blend function presets for ease of use.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum BlendPreset {
    /// When combining two fragments, add their values together, saturating at 1.0
    Add,
    /// When combining two fragments, multiply their values together.
    Multiply,
    /// When combining two fragments, add the value of the source times its alpha channel with the
    /// value of the destination multiplied by the inverse of the source alpha channel. Has the
    /// usual transparency effect: mixes the two colors using a fraction of each one specified by
    /// the alpha of the source.
    Alpha,
    /// When combining two fragments, subtract the destination color from a constant color
    /// using the source color as weight. Has an invert effect with the constant color
    /// as base and source color controlling displacement from the base color.
    /// A white source color and a white value results in plain invert.
    /// The output alpha is same as destination alpha.
    Invert,
}

impl DrawState {
    /// Create a default `DrawState`. Uses counter-clockwise winding, culls the backface of each
    /// primitive, and does no scissor/stencil/depth/blend/color masking.
    pub fn new() -> DrawState {
        DrawState {
            primitive: state::Primitive {
                front_face: FrontFace::CounterClockwise,
                method: RasterMethod::Fill(CullFace::Back),
                offset: None,
            },
            multi_sample: None,
            scissor: None,
            stencil: None,
            depth: None,
            blend: [None; MAX_COLOR_TARGETS],
            ref_values: Default::default(),
        }
    }

    /// Return a target mask that contains all the planes required by this state.
    pub fn get_target_mask(&self) -> Mask {
        use target as t;
        (if self.stencil.is_some()  {t::STENCIL} else {Mask::empty()}) |
        (if self.depth.is_some()    {t::DEPTH}   else {Mask::empty()}) |
        (if self.blend[0].is_some() {t::COLOR0}  else {Mask::empty()}) |
        (if self.blend[1].is_some() {t::COLOR1}  else {Mask::empty()}) |
        (if self.blend[2].is_some() {t::COLOR2}  else {Mask::empty()}) |
        (if self.blend[3].is_some() {t::COLOR3}  else {Mask::empty()})
    }

    /// Enable multi-sampled rasterization
    pub fn multi_sample(mut self) -> DrawState {
        self.multi_sample = Some(state::MultiSample);
        self
    }

    /// Set the stencil test to a simple expression
    pub fn stencil(mut self, fun: state::Comparison, value: Stencil) -> DrawState {
        let side = state::StencilSide {
            fun: fun,
            mask_read: Stencil::max_value(),
            mask_write: Stencil::max_value(),
            op_fail: StencilOp::Keep,
            op_depth_fail: StencilOp::Keep,
            op_pass: StencilOp::Keep,
        };
        self.stencil = Some(state::Stencil {
            front: side,
            back: side,
        });
        self.ref_values.stencil = (value, value);
        self
    }

    /// Set the depth test with the mask
    pub fn depth(mut self, fun: state::Comparison, write: bool) -> DrawState {
        self.depth = Some(state::Depth {
            fun: fun,
            write: write,
        });
        self
    }

    /// Set the scissor
    pub fn scissor(mut self, x: u16, y: u16, w: u16, h: u16) -> DrawState {
        self.scissor = Some(target::Rect { x: x, y: y, w: w, h: h });
        self
    }

    /// Set the blend mode to one of the presets
    pub fn blend(mut self, preset: BlendPreset) -> DrawState {
        self.blend[0] = Some(match preset {
            BlendPreset::Add => state::Blend {
                color: state::BlendChannel {
                    equation: Equation::Add,
                    source: state::Factor::One,
                    destination: state::Factor::One,
                },
                alpha: state::BlendChannel {
                    equation: Equation::Add,
                    source: state::Factor::One,
                    destination: state::Factor::One,
                },
                mask: state::MASK_ALL,
            },
            BlendPreset::Multiply => state::Blend {
                color: state::BlendChannel {
                    equation: Equation::Add,
                    source: state::Factor::ZeroPlus(BlendValue::DestColor),
                    destination: state::Factor::Zero,
                },
                alpha: state::BlendChannel {
                    equation: Equation::Add,
                    source: state::Factor::ZeroPlus(BlendValue::DestAlpha),
                    destination: state::Factor::Zero,
                },
                mask: state::MASK_ALL,
            },
            BlendPreset::Alpha => state::Blend {
                color: state::BlendChannel {
                    equation: Equation::Add,
                    source: state::Factor::ZeroPlus(BlendValue::SourceAlpha),
                    destination: state::Factor::OneMinus(BlendValue::SourceAlpha),
                },
                alpha: state::BlendChannel {
                    equation: Equation::Add,
                    source: state::Factor::One,
                    destination: state::Factor::One,
                },
                mask: state::MASK_ALL,
            },
            BlendPreset::Invert => state::Blend {
                color: state::BlendChannel {
                    equation: Equation::Sub,
                    source: state::Factor::ZeroPlus(state::BlendValue::ConstColor),
                    destination: state::Factor::ZeroPlus(state::BlendValue::SourceColor),
                },
                alpha: state::BlendChannel {
                    equation: state::Equation::Add,
                    source: state::Factor::Zero,
                    destination: state::Factor::One,
                },
                mask: state::MASK_ALL,
            },
        });
        self
    }
}
