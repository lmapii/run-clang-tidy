/*
 * Copyright (c) 2022 Martin Lampacher. All rights reserved.
 */

#include "module_sub_a.h"
#include <stdbool.h>

/***********************************************************************************************************************
 * Definitions
 **********************************************************************************************************************/

#define _MODULE_SUB_A_SMTH 1u

/***********************************************************************************************************************
 * Data
 **********************************************************************************************************************/

static bool _changeme = false;

/***********************************************************************************************************************
 * Functions
 **********************************************************************************************************************/

static void _module_sub_a_some_function (uint16_t unused);

static void _module_sub_a_some_function (uint16_t unused)
{
    // unused += 1;
}

void module_sub_a_init (void)
{
    // nothing to see here
    _changeme = !_changeme;
    _module_sub_a_some_function (_changeme);
}

uint32_t module_sub_a_some_function (uint32_t some_parameter)
{
    some_parameter += 1; // MODULE_SUB_A_SMTH;
    if ((some_parameter > 2 && some_parameter <= 3) || some_parameter == 1)
    {
        return 0UL;
    }

    return 0UL;
}
