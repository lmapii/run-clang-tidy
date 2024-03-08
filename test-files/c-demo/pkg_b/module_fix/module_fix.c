/*
 * Copyright (c) 2022 Martin Lampacher. All rights reserved.
 */

#include "module_fix.h"

/***********************************************************************************************************************
 * Functions
 **********************************************************************************************************************/

static volatile unsigned int _fix_dummy = 0U;

void module_fix_init (void)
{
    _fix_dummy = MODULE_FIX_EXPRESSION(32U, 64U);
    (void) _fix_dummy;
}
