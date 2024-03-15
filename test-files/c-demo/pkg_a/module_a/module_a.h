/**
 * \file module_a.h
 *
 * \brief dummy module
 *
 * Copyright (c) 2022 Martin Lampacher. All rights reserved.
 */

#ifndef _MODULE_A_H_
#define _MODULE_A_H_

/***********************************************************************************************************************
 * Includes
 **********************************************************************************************************************/

#include "module_b.h"
#include "std_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/***********************************************************************************************************************
 * Definitions
 **********************************************************************************************************************/

#define MODULE_A_SMTH 0x3

#define CALC(_value_2, _value_3) \
    do                           \
    {                            \
        if ((_value_2) != FALSE) \
        {                        \
            (_value_3) *= 5;     \
        }                        \
        else                     \
        {                        \
            (_value_3) = 23;     \
        }                        \
    } while (0);

// typedef enum
// {
//     E_MODULE_A_SOME_VALUE_0,
//     E_MODULE_A_SOME_VALUE_1
// } e_module_a_enum;

/***********************************************************************************************************************
 * Functions
 **********************************************************************************************************************/

void module_a_init (void);

#ifdef __cplusplus
}
#endif

#endif /* _MODULE_A_H_ */
