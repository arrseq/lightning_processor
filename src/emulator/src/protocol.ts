import type { Commands } from "./protocol/command";

export * as command from "./protocol/command";

// Websocket protocol interface for the x54 emulator backend system.
export default class Protocol {
    public websocket: WebSocket;
    public on_open_listener: () => void = () => {};
    public on_close_listener: () => void = () => {};

    public constructor() {
        this.websocket = new WebSocket("ws://127.0.0.1:15147");

        let protocol = this;

        this.websocket.binaryType = "arraybuffer";
        this.websocket.onopen = () => protocol.on_open();
        this.websocket.onclose = () => protocol.on_close;
        this.websocket.onerror = () => protocol.on_error;
    }

    public send_raw(buffer: ArrayBuffer) {
        this.websocket?.send(buffer);
    }

    public send_raw_command(command: Commands, extension_bytes: ArrayBuffer = new ArrayBuffer(0)): Promise<ArrayBuffer> {
        return new Promise((res, rej) => {
            let c_buffer = new ArrayBuffer(4 /* u32 command code */);
            let c_view = new DataView(c_buffer);
            c_view.setUint32(0, command, false);
    
            let main_buffer = new ArrayBuffer(c_buffer.byteLength + extension_bytes.byteLength);
            let byte_buffer = new Uint8Array(main_buffer);
            byte_buffer.set(new Uint8Array(c_buffer));
            byte_buffer.set(new Uint8Array(extension_bytes), c_buffer.byteLength);
    
            this.send_raw(main_buffer);
        });
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