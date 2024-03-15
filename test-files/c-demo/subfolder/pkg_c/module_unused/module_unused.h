/**
 * \file module_unused.h
 *
 * \brief dummy module
 *
 * Copyright (c) 2022 Martin Lampacher. All rights reserved.
 */

#ifndef _MODULE_UNUSED_H_
#define _MODULE_UNUSED_H_

/***********************************************************************************************************************
 * Includes
 **********************************************************************************************************************/

#include "std_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/***********************************************************************************************************************
 * Definitions
 **********************************************************************************************************************/

typedef enum
{
    E_MODULE_UNUSED_SOME_VALUE_0,
    this_does_not_match_the_style,
} e_module_unused_a_enum;

#define MODULE_UNUSED_SMTH 0x0

/***********************************************************************************************************************
 * Functions
 **********************************************************************************************************************/

void module_unused_init (uint8 some_parameter);

#ifdef __cplusplus
}
#endif

#endif /* _MODULE_UNUSED_H_ */
