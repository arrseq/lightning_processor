@end process:
    terminate

start:
    branch           , calculate

calculate:
    load immediate   , general purpose 0                   , 10
    load immediate   , general purpose 1                   , 25
    add              , general purpose 2, general purpose 1, general purpose 2
    branch           , display result

display result:
    load interconnect, general purpose 2
    interupt         , @end process