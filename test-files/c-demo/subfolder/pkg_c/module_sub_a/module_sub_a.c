/*
 * Copyright (c) 2022 Martin Lampacher. All rights reserved.
 */

#include "module_sub_a.h"

/***********************************************************************************************************************
 * Definitions
 **********************************************************************************************************************/

#define _MODULE_SUB_A_SMTH 1u

/***********************************************************************************************************************
 * Data
 **********************************************************************************************************************/

static boolean _changeme = FALSE;

/***********************************************************************************************************************
 * Functions
 **********************************************************************************************************************/

static void _module_sub_a_some_function (uint16 unused);

static void _module_sub_a_some_function (uint16 unused)
{
    // unused += 1;
}

void module_sub_a_init (void)
{
    // nothing to see here
    _changeme = !_changeme;
    _module_sub_a_some_function (_changeme);
}

uint32 module_sub_a_some_function (uint32 some_parameter)
{
    some_parameter += 1; // MODULE_SUB_A_SMTH;
    if ((some_parameter > 2 && some_parameter <= 3) || some_parameter == 1)
    {
        return 0UL;
    }

    return 0UL;
}
