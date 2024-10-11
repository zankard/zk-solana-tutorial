import * as snarkjs from "snarkjs";
import path from "path";
import { expect, test } from 'vitest'
const { buildBn128, utils } = require("ffjavascript");
const { unstringifyBigInts } = utils;

const wasmPath = path.join(__dirname, "../build/circuits", "multiplier_js/multiplier.wasm");
const zkeyPath = path.join(__dirname, "../build", "multiplier_final.zkey");

function g1Uncompressed(curve, p1Raw) {
    let p1 = curve.G1.fromObject(p1Raw);
    let buff = new Uint8Array(64);
    curve.G1.toRprUncompressed(buff, 0, p1);

    return Buffer.from(buff);
}

function g2Uncompressed(curve, p2Raw) {
    let p2 = curve.G2.fromObject(p2Raw);
    let buff = new Uint8Array(128);
    curve.G2.toRprUncompressed(buff, 0, p2);
    return Buffer.from(buff);
}

function to32ByteBuffer(num: BigInt) {
    const hexString = num.toString(16).padStart(64, '0');
    const buffer = Buffer.from(hexString, 'hex');
    return buffer;
}

test("multiplier test", async () => {
    let input = { "a": 3, "b": 4 };
    let { proof, publicSignals } = await snarkjs.groth16.fullProve(input, wasmPath, zkeyPath);

    console.log(publicSignals);
    console.log(proof);

    let curve = await buildBn128();
    let proofProc = unstringifyBigInts(proof);
    publicSignals = unstringifyBigInts(publicSignals);

    const pi_a = g1Uncompressed(curve, proofProc.pi_a);
    let pi_a_0_u8_array = Array.from(pi_a);
    console.log(pi_a_0_u8_array);

    const pi_b = g2Uncompressed(curve, proofProc.pi_b);
    let pi_b_0_u8_array = Array.from(pi_b);
    console.log(pi_b_0_u8_array.slice(0, 64));
    console.log(pi_b_0_u8_array.slice(64, 128));

    const pi_c = g1Uncompressed(curve, proofProc.pi_c);
    let pi_c_0_u8_array = Array.from(pi_c);
    console.log(pi_c_0_u8_array);

    const publicSignalsBuff = to32ByteBuffer(BigInt(publicSignals[0]));
    let public_signal_0_u8_array = Array.from(publicSignalsBuff);
    console.log(public_signal_0_u8_array);
});
