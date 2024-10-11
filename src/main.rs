mod verifying_key;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate};
    //use ark_std::io::prelude::*;
    //use groth16_solana::groth16::Groth16Verifyingkey;
    use std::ops::Neg;
    use groth16_solana::groth16::Groth16Verifier;
    use crate::verifying_key;

    type G1 = ark_bn254::g1::G1Affine;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    fn change_endianness(bytes: &[u8]) -> Vec<u8> {
        let mut vec = Vec::new();
        for b in bytes.chunks(32) {
            for byte in b.iter().rev() {
                vec.push(*byte);
            }
        }
        vec
    }

    #[test]
    fn proof2_verification_should_succeed() {
        const PROOF_2: [u8; 256] = [
            32, 109, 30, 98, 15, 11, 179, 155, 24, 45, 50, 60, 140, 163, 62, 87, 200, 64, 191, 119,
            65, 40, 241, 90, 144, 13, 104, 248, 42, 238, 202, 109, 45, 190, 104, 161, 241, 138,
            105, 176, 65, 89, 204, 103, 64, 149, 79, 133, 64, 117, 17, 252, 86, 216, 254, 234, 98,
            27, 200, 214, 170, 179, 173, 183, 11, 63, 248, 190, 69, 188, 209, 172, 116, 172, 9,
            175, 197, 164, 63, 137, 39, 224, 123, 126, 207, 99, 156, 9, 253, 41, 175, 254, 211, 55,
            178, 238, 39, 13, 68, 50, 51, 68, 119, 54, 183, 19, 13, 98, 167, 185, 206, 12, 12, 169,
            225, 69, 193, 70, 158, 25, 128, 235, 250, 73, 51, 11, 28, 107, 3, 71, 136, 59, 46, 170,
            99, 199, 98, 34, 110, 108, 75, 1, 199, 23, 113, 116, 247, 164, 213, 247, 63, 216, 106,
            35, 110, 47, 177, 231, 242, 181, 7, 59, 31, 139, 158, 219, 76, 46, 59, 17, 25, 66, 84,
            63, 199, 66, 86, 85, 66, 97, 221, 240, 59, 159, 106, 53, 253, 43, 13, 22, 45, 86, 30,
            152, 170, 103, 94, 91, 145, 238, 179, 184, 13, 49, 144, 111, 242, 236, 161, 169, 93,
            97, 228, 49, 36, 78, 61, 200, 229, 140, 48, 169, 26, 7, 24, 4, 244, 17, 234, 37, 149,
            125, 238, 19, 248, 246, 41, 166, 154, 25, 57, 190, 139, 51, 201, 164, 155, 60, 98, 186,
            119, 248, 214, 26, 231, 41,
        ];
        const PUBLIC_INPUTS_2: [[u8; 32]; 1] = [[
            0, 0, 0, 0,  0, 0, 0, 0, 0,
            0, 0, 0, 0,  0, 0, 0, 0, 0,
            0, 0, 0, 0,  0, 0, 0, 0, 0,
            0, 0, 0, 0, 12
        ]];
        let proof_a: G1 = G1::deserialize_with_mode(
            &*[&change_endianness(&PROOF_2[0..64]), &[0u8][..]].concat(),
            Compress::No,
            Validate::Yes,
        )
            .unwrap();

        let mut proof_a_neg = [0u8; 65];
        proof_a
            .neg()
            .x
            .serialize_with_mode(&mut proof_a_neg[..32], Compress::No)
            .unwrap();

        proof_a
            .neg().y.serialize_with_mode(&mut proof_a_neg[32..], Compress::No).unwrap();
        let proof_a = change_endianness(&proof_a_neg[..64]).try_into().unwrap();
        let proof_b = PROOF_2[64..192].try_into().unwrap();
        let proof_c = PROOF_2[192..256].try_into().unwrap();

        let mut verifier = Groth16Verifier::new(&proof_a, &proof_b, &proof_c,
        &PUBLIC_INPUTS_2, &verifying_key::VERIFYINGKEY).unwrap();
        verifier.verify().unwrap();
    }
}
