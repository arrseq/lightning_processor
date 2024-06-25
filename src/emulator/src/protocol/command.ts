export let U32_MAX = Math.pow(2, 32)-1;

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

export enum Commands {
    // All accept a 64 bit address
    Memory__ReadByteFrame,
    Memory__ReadWordFrame,
    Memory__ReadDualFrame,
    Memory__ReadQuadFrame
}

export enum Errors {
    Memory__InvalidAddress,
    Memory__PageFault
}

// How many bytes is the L result.
export function get_l_size(command: Commands): number {
    switch (command) {
        case Commands.Memory__ReadByteFrame:
            return 1;
        case Commands.Memory__ReadDualFrame:
            return 2;
        case Commands.Memory__ReadWordFrame:
            return 4;
        case Commands.Memory__ReadQuadFrame:
            return 8;
    }
}

export function read_l_result_sized(response: Uint8Array, size: number): number[] {
    return [];
}

export function read_l_result(response: Uint8Array, command: Commands): number[] {
    return read_l_result_sized(response, get_l_size(command));
}