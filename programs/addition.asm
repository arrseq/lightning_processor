@end process:
    terminate

start:
    divert           , calculate

calculate:
    load immediate   , general purpose 0                   , 10
    load immediate   , general purpose 1                   , 25
    add              , general purpose 2, general purpose 1, general purpose 2
    divert           , display result

display result:
    load interconnect, general purpose 2
    interupt         , @end process