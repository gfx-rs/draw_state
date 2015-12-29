// Copyright 2015 The Gfx-rs Developers.
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

//! State presets

/// Blending preset modes.
pub mod blend {
	use state::{Blend, BlendChannel, BlendValue, Equation, Factor, MASK_ALL};

	/// When combining two fragments, add their values together, saturating at 1.0
	pub const ADD: Blend = Blend {
	    color: BlendChannel {
            equation: Equation::Add,
            source: Factor::One,
            destination: Factor::One,
        },
        alpha: BlendChannel {
            equation: Equation::Add,
            source: Factor::One,
            destination: Factor::One,
        },
        mask: MASK_ALL,
    };

    /// When combining two fragments, multiply their values together.
    pub const MULTIPLY: Blend = Blend {
        color: BlendChannel {
            equation: Equation::Add,
            source: Factor::ZeroPlus(BlendValue::DestColor),
            destination: Factor::Zero,
        },
        alpha: BlendChannel {
            equation: Equation::Add,
            source: Factor::ZeroPlus(BlendValue::DestAlpha),
            destination: Factor::Zero,
        },
        mask: MASK_ALL,
    };

    /// When combining two fragments, add the value of the source times its alpha channel with the
    /// value of the destination multiplied by the inverse of the source alpha channel. Has the
    /// usual transparency effect: mixes the two colors using a fraction of each one specified by
    /// the alpha of the source.
    pub const ALPHA: Blend = Blend {
        color: BlendChannel {
            equation: Equation::Add,
            source: Factor::ZeroPlus(BlendValue::SourceAlpha),
            destination: Factor::OneMinus(BlendValue::SourceAlpha),
        },
        alpha: BlendChannel {
            equation: Equation::Add,
            source: Factor::One,
            destination: Factor::One,
        },
        mask: MASK_ALL,
    };

    /// When combining two fragments, subtract the destination color from a constant color
    /// using the source color as weight. Has an invert effect with the constant color
    /// as base and source color controlling displacement from the base color.
    /// A white source color and a white value results in plain invert.
    /// The output alpha is same as destination alpha.
    pub const INVERT: Blend = Blend {
        color: BlendChannel {
            equation: Equation::Sub,
            source: Factor::ZeroPlus(BlendValue::ConstColor),
            destination: Factor::ZeroPlus(BlendValue::SourceColor),
        },
        alpha: BlendChannel {
            equation: Equation::Add,
            source: Factor::Zero,
            destination: Factor::One,
        },
        mask: MASK_ALL,
    };
}
