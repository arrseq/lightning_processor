; This routine is an interupt and recognized by the assembler.
; This will be referenced by the interupt table.
safe_interupt:
    liq, us00, [save_state]
    dvt, us00 ; Try to save the safe state

save_state:
    ; Try to push all of the program registers to stack, this will save the context.
    ; Since there is no program stack or buffer we have to save it to the unsafe stack.
    
    ; Store gn00
    qtm, gn00, stpr
    sub, stpr, true

    ; Store gn01
    qtm, gn01, stpr
    sub, stpr, true

    ; End
    trm

execute_program:
    liq, us00, [program]
    lib, us01, #10
    safe, us00, us01 ; Safe context switch to safe_block for 10 clock cycles
    ; It ends right here

start:
    ; Setup stack
    cln, stpr, mmry ; Take the memory amount in bytes and set at stack pointer

    ; Run the program and wait for it to finish its first time share
    lib, us00, [execute_program]
    dvt, us00 ; Load and execute the program

; --------- Program Half --------- 
; - Diverting to this program in unsafe mode will run it in unsafe. 
;   It will also not end.
; - Safe calling this program in unsafe mode will run it for 10 
;   instruction cycles and then return to the safe interupt.

program:
    lib, gn00, #100 ; Load immediate byte into gn00

    ; Infinite loop
    lib, gn01, [threadlock]
    dvt, gn01 ; Start thread lock

threadlock:
    ; Initialize thread lock, store the diversion addresses
    lib, gn01, [threadlock_inner] ; Address to thread lock itself
    dvt, gn02 ; Enter the thread lock

; This exists so that the thread lock pointer would not have to be
; reinitialized for each run of the lock. This is designed to be a
; minimal expense lock
threadlock_inner:
    dvt, gn01