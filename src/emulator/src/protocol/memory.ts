import { Commands, User, generate_bool, generate_u64, mash } from "./command";

export default class Memory extends User {
    public read_byte_frame(address: ArrayBuffer, translate: boolean): Promise<ArrayBuffer> {
        return this.get_protocol().send_command(Commands.Memory__ReadByteFrame, mash([ address, generate_bool(translate) ]));
    }
}