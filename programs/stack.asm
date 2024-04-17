start:
    // Allocate an 8 and 4 byte integer. Load memory is a compound instruction.
    load to memory  , stack pointer    , #FF FF FF FF FF FF FF FF
    load to memory  , stack pointer    , #FF AB FF AB
    subtract        , stack pointer    , quad word
    subtract        , stack pointer    , double word

    // Store the 2 integers into registers.
    load from memory, general purpose 0, stack pointer - double word
    load from memory, general purpose 0, stack pointer - quad word
