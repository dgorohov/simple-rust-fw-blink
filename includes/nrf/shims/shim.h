#ifndef __NRF_SHIM_H__
#define __NRF_SHIM_H__

#pragma GCC diagnostic ignored "-Wimplicit-function-declaration"

#include "nrf_delay.h"
#include "nrfx_common.h"

void _nrf_delay_ms(uint32_t number_of_ms);

#endif // __NRF_SHIM_H__
