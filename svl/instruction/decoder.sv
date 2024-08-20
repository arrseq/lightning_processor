module decoder(
    input logic [31:0] u32
);
    logic [3:0] opc;
    logic [27:0] oprands;

    function void prin_op(input logic [3:0] opc);
        case (opc)
            4'h00: $display("push");
            4'h01: $display("pop");
            4'h02: $display("unary");
            4'h03: $display("binary");
            4'h04: $display("mov");
            4'h05: $display("blvec");
            4'h06: $display("lodi");
            4'h07: $display("lmem");
            4'h08: $display("lmem_base");
            4'h09: $display("jmp");
            4'h0A: $display("jmp_base");
            4'h0B: $display("mnemonic");
            4'h0C: $display("lock");
            default: $display("mnemonic");
        endcase
    endfunction

    function void dec(input logic [31:0] u32);
        opc = u32[3:0];
        oprands = u32[31:4];
        prin_op(opc);
    endfunction

    initial begin
        dec(u32);
    end

endmodule