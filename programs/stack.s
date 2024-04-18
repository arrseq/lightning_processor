start:
    // Allocate 4 byte unsigned number. #FF AB FF CF
    ltm, stpr, #FF 
    sub, stpr, true

    ltm, stpr, #AB 
    sub, stpr, true

    ltm, stpr, #FF 
    sub, stpr, true

    ltm, stpr, #CF 
    sub, stpr, true
    
    // Store the variable into gn00
    cln, gn01, stpr
    sub, gn01, dwrd
    lfm, gn00, gn01, dwrd // Load from memory(Destination, Address, Bytes) 

    // Send to processor output
    lic, gn00
