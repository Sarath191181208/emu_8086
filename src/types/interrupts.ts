// the interface could be any one of the following:
// Print : 0x00
// Read : [0x01, 0x02]
// Write : [0x03, 0x04]

export type InterruptType = "Print" | "Read" | "Write";

export type Interrupt = {
    type: InterruptType;
    value: any;
}
