#include "../bindings.h"

#include "shim.h"

void _nrf_delay_ms(uint32_t number_of_ms) { nrf_delay_ms(number_of_ms); }
