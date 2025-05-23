// build-pass
// compile-flags: -Ctarget-feature=+ShaderNonUniform,+ext:SPV_EXT_descriptor_indexing

use spirv_std::spirv;
use spirv_std::{Image, arch};

#[spirv(fragment)]
pub fn main(
    #[spirv(descriptor_set = 0, binding = 0)] image: &Image!(2D, format = rgba32f, sampled),
    output: &mut glam::Vec4,
) {
    let texel = image.fetch(glam::IVec2::new(0, 1));
    *output = texel;
}
