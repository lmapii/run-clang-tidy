/*
 * Copyright (c) 2022 Martin Lampacher. All rights reserved.
 */

#include "module_b.h"
#include "module_untidy.h"

/***********************************************************************************************************************
 * Definitions
 **********************************************************************************************************************/

#define _MAXLOOP 1234UL

/***********************************************************************************************************************
 * Data
 **********************************************************************************************************************/

static volatile uint8 _some_variable[] = {MODULE_UNTIDY_SMTH, 2, 3};

/***********************************************************************************************************************
 * Functions
 **********************************************************************************************************************/

int main (int argc, const char *argv[]) // NOLINT : unused argument argv
{
    // uint8 i = 0;
    // module_a_init ();

    module_b_init ();
    module_c_init ();

    _some_variable[0] = 123; // NOLINT: magic number
    _some_variable[0] = 2;

    for (uint32 i = 0; i < _MAXLOOP; i++)
    {
        _some_variable[0] += 1;
    }
}
