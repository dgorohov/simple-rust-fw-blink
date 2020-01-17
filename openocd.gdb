target extended-remote :3333
monitor arm semihosting enable
monitor arm semihosting_cmdline name 4 3 2
load
break main
layout src
continue