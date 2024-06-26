import type Protocol from "../protocol";

export let U32_MAX = Math.pow(2, 32)-1;

export function mash(blocks: ArrayBuffer[]): ArrayBuffer {
    let full_len = 0;
    blocks.forEach((b) => full_len += b.byteLength);

    let target = new ArrayBuffer(full_len);
    let u8 = new Uint8Array(target);

    let offset = 0;
    blocks.forEach((block, index) => {
        let b_u8 = new Uint8Array(block);
        u8.set(b_u8, offset);

        offset += b_u8.byteLength;
    });

    return target;
}

// Combine to u32's into a u64 array buffer.
export function generate_u64_array(first: Uint8Array, second: Uint8Array): Uint8Array {
    let u64 = new Uint8Array(8);
    u64.set(first);
    u64.set(second, first.byteLength);
    return u64;
}

export function generate_u64(first: number, second: number): Uint8Array {
    let first_u8 = new ArrayBuffer(4);
    let second_u8 = new ArrayBuffer(4);
    let i_first = new DataView(first_u8);
    let i_second = new DataView(second_u8);

    i_first.setUint32(0, first, false);
    i_second.setUint32(0, second, false);

    return generate_u64_array(new Uint8Array(first_u8), new Uint8Array(second_u8));
}

export function generate_bool(state: boolean): Uint8Array {
    let boolean = new ArrayBuffer(1);
    let buffer = new DataView(boolean);
    buffer.setUint8(0, state ? 1 : 0);
    return new Uint8Array(boolean);
}

export enum Commands {
    // All accept a 64 bit address
    Memory__ReadByteFrame,
    Memory__ReadWordFrame,
    Memory__ReadDualFrame,
    Memory__ReadQuadFrame,

    // Test System
    Test__VideoRedNoise,
    Test__VideoRedNoise__SetDimension
}

export enum Errors {
    Memory__InvalidAddress,
    Memory__PageFault,
}

// How many bytes is the L result.
// export function get_l_size(command: Commands): number {
//     switch (command) {
//         case Commands.Memory__ReadByteFrame:
//             return 1;
//         case Commands.Memory__ReadDualFrame:
//             return 2;
//         case Commands.Memory__ReadWordFrame:
//             return 4;
//         case Commands.Memory__ReadQuadFrame:
//             return 8;
//         case Commands.Test__VideoRedNoise:
//             return -0; // Very Large Array
//     }
// }

export function read_l_result_sized(response: Uint8Array, size: number): number[] {
    return [];
}

/**
 * @deprecated DO NOT USE. Size cannot always be known.
 */
// export function read_l_result(response: Uint8Array, command: Commands): number[] {
//     return read_l_result_sized(response, get_l_size(command));
// }

export abstract class User {
    private proto_inner: Protocol;
    
    public constructor(protocol: Protocol) {
        this.proto_inner = protocol;
    }
    
    protected get_protocol(): Protocol {
        return this.proto_inner;
    }
}