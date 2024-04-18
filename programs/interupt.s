interupt +int:
    lib, gn00, #FF
    lic, gn00

start:
    // Interupt calls are always checked to make sure 
    int, interupt 