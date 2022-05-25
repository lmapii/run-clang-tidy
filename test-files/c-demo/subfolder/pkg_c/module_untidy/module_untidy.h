/**
 * \file module_untidy.h
 *
 * \brief dummy module
 *
 * Copyright (c) 2022 Martin Lampacher. All rights reserved.
 */

#ifndef _MODULE_UNTIDY_H_
#define _MODULE_UNTIDY_H_

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
    E_MODULE_UNTIDY_SOME_VALLUE_0,
    this_does_not_match_the_style,
} e_module_untidy_a_enum;

// #define MODULE_UNTIDY_SMTH 2uL // violates suffix convention in the resolved file
#define MODULE_UNTIDY_SMTH 2UL

/***********************************************************************************************************************
 * Functions
 **********************************************************************************************************************/

void module_untidy_init (uint8 some_arg);

#ifdef __cplusplus
}
#endif

#endif /* _MODULE_UNTIDY_H_ */
