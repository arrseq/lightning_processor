import { U32_MAX, type Commands } from "./protocol/command";

function bytes_to_u32(bytes: number[]): number {
    let value = 0;
    for (var i = 0; i < bytes.length; i++) {
        value = (value << 8) | bytes[i];
    }
    return value;
}

type QueueWaiter = {
    [index: string]: (buf: ArrayBuffer) => void
}

// Websocket protocol interface for the x54 emulator backend system.
export default class Protocol {
    public websocket: WebSocket;
    public on_open_listener: () => void = () => {};
    public on_close_listener: () => void = () => {};
    private waiting_on: QueueWaiter = {};

    public constructor() {
        this.websocket = new WebSocket("ws://127.0.0.1:15147");

        let protocol = this;

        this.websocket.binaryType = "arraybuffer";
        this.websocket.onopen = () => protocol.on_open();
        this.websocket.onclose = () => protocol.on_close;
        this.websocket.onerror = () => protocol.on_error;

        this.websocket.onmessage = (message) => {
            let buffer = message.data;
            let u8b = new Uint8Array(buffer);
            let id = bytes_to_u32([ u8b.at(0)!, u8b.at(1)!, u8b.at(2)!, u8b.at(3)! ]);

            let result = new ArrayBuffer(u8b.byteLength - 4);
            let result_u8 = new Uint8Array(result);

            

            this.waiting_on[id](result);
        }
    }

    public send_raw(buffer: ArrayBuffer) {
        this.websocket?.send(buffer);
    }

    // Encodes as [COMMAND32, ID32, ...]
    // Response arrives as [ID32, ...]
    public send_raw_command(command: Commands, id: number, extension_bytes: ArrayBuffer = new ArrayBuffer(0)): Promise<ArrayBuffer> {
        return new Promise((res, rej) => {
            let c_buffer = new ArrayBuffer(4 /* u32 command code */);
            let c_view = new DataView(c_buffer);
            c_view.setUint32(0, command, false);

            let i_buffer = new ArrayBuffer(4 /* u32 invocation id */);
            let i_view = new DataView(i_buffer);
            i_view.setUint32(0, id, false);
    
            let main_buffer = new ArrayBuffer(c_buffer.byteLength + extension_bytes.byteLength + i_buffer.byteLength);
            let byte_buffer = new Uint8Array(main_buffer);
            byte_buffer.set(new Uint8Array(c_buffer));
            byte_buffer.set(new Uint8Array(i_buffer), c_buffer.byteLength)
            byte_buffer.set(new Uint8Array(extension_bytes), c_buffer.byteLength + i_buffer.byteLength);

            Object.defineProperty(this.waiting_on, id + "", {
                value: (b: ArrayBuffer) => res(b),
                writable: true
            });

            this.send_raw(main_buffer);
        });
    }

    private idgen(prev: number = -1): number {
        let pos_id = Math.abs(prev + 1 + Math.round(Math.random() * 25));
        if (Object.keys(this.waiting_on).includes(pos_id + "")) {
            return this.idgen(prev);
        }

        return pos_id;
    }

    public send_command(command: Commands, extension_bytes: ArrayBuffer = new ArrayBuffer(0)): Promise<ArrayBuffer> {
        return this.send_raw_command(command, this.idgen(), extension_bytes);
    }

    public destroy() {
        this.websocket?.close();
    }

    private on_open() {
        console.log("[x54] Connected to backend.");
        this.on_open_listener();
    }

    private on_close() {
        console.log("[x54] Connection to backend was terminated.");
        this.on_close_listener();
    }

    private on_error(error: Event) {
        console.error(`[x54] Error in protocol raw websocket. Error.message=${error}`);
    }
}

export * as command from "./protocol/command";
export * as memory from "./protocol/memory";