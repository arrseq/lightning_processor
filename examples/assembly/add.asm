increment:
    inc r1
    add r1+1, r1, r1

arithmetic:
    ; multiply and add. r1=1; r1*=1; r1+=r1+1.
    inc r1
    mul r1, r1, r0+1
    add r1, r1, r1+1