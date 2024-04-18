start:
    // Add 5 + 7 while storing them in byte sized registers. 
    lib, gn00, #05
    lib, gn01, #07
    add, gn02, gn00, gn01
    lic, gn02