// build-pass
// compile-flags: -C target-feature=+ImageQuery,+ShaderNonUniform,+ext:SPV_EXT_descriptor_indexing

use spirv_std::spirv;
use spirv_std::{Image, arch};

#[spirv(fragment)]
pub fn main(
    #[spirv(descriptor_set = 0, binding = 0)] image: &Image!(2D, type=f32, sampled),
    output: &mut u32,
) {
    *output = image.query_levels();
}
