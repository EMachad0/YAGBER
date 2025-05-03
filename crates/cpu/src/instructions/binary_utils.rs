/// Check if the opcode matches the mask
/// mask is the u8 with ones in the positions that should be ones
/// nmask is the u8 with ones in the positions that should be zeros
pub(super) fn match_mask(opcode: u8, mask: u8, nmask: u8) -> bool {
    (opcode & mask) == mask && (opcode & nmask) == 0
}
