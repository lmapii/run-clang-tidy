/**
 * \file module_sub_a.h
 *
 * \brief dummy module
 *
 * Copyright (c) 2022 Martin Lampacher. All rights reserved.
 */

#ifndef _MODULE_SUB_A_H_
#define _MODULE_SUB_A_H_

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

// clang-tidy will only complain once it is used ...
#define MODULE_SUB_A_SMTH 1ul

// typedef enum
// {
//     E_MODULE_SUB_A_SOME_VALUE_0,
//     E_MODULE_SUB_A_SOME_VALUE_1
// } e_module_sub_a_enum;

/***********************************************************************************************************************
 * Functions
 **********************************************************************************************************************/

void module_sub_a_init (void);

uint32 module_sub_a_some_function (uint32 some_parameter);

#ifdef __cplusplus
}
#endif

#endif /* _MODULE_SUB_A_H_ */
