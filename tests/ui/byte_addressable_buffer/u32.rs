// build-pass
// compile-flags: -Ctarget-feature=+ShaderNonUniform,+ext:SPV_EXT_descriptor_indexing

use spirv_std::ByteAddressableBuffer;
use spirv_std::spirv;

#[spirv(fragment)]
pub fn load(#[spirv(descriptor_set = 0, binding = 0, storage_buffer)] buf: &[u32], out: &mut u32) {
    unsafe {
        let buf = ByteAddressableBuffer::from_slice(buf);
        *out = buf.load(5);
    }
}

#[spirv(fragment)]
pub fn load_mut(
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] buf: &mut [u32],
    out: &mut u32,
) {
    unsafe {
        let buf = ByteAddressableBuffer::from_mut_slice(buf);
        *out = buf.load(5);
    }
}

#[spirv(fragment)]
pub fn store(
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] buf: &mut [u32],
    #[spirv(flat)] val: u32,
) {
    unsafe {
        let mut buf = ByteAddressableBuffer::from_mut_slice(buf);
        buf.store(5, val);
    }
}
