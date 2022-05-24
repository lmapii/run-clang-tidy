/**
 * \file module_b.h
 *
 * \brief dummy module
 *
 * Copyright (c) 2022 Martin Lampacher. All rights reserved.
 */

#ifndef _MODULE_B_H_
#define _MODULE_B_H_

/***********************************************************************************************************************
 * Includes
 **********************************************************************************************************************/

#include "std_types.h"
// used to test sub-includes (changing module_c.h > main.c must be re-compiled)
#include "module_c.h"

#ifdef __cplusplus
extern "C" {
#endif

/***********************************************************************************************************************
 * Definitions
 **********************************************************************************************************************/

typedef enum
{
    E_SOME_VALLUE_0,
    E_SOME_VALULE_1
} some_enum;

#define MODULE_B_SMTH 0x1

/***********************************************************************************************************************
 * Functions
 **********************************************************************************************************************/

void module_b_init (void);

#ifdef __cplusplus
}
#endif

#endif /* _MODULE_B_H_ */
