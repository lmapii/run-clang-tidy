/*
 * Copyright (c) 2022 Martin Lampacher. All rights reserved.
 */

#include "module_b.h"
#include "module_a.h"

/***********************************************************************************************************************
 * Functions
 **********************************************************************************************************************/

void module_b_init (void)
{
    module_a_init ();
    // nothing to see here
}
