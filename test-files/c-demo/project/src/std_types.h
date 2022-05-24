/**
 * \file std_types.h
 *
 * \brief Standard type definitions.
 *
 * Copyright (c) Martin Lampacher. All rights reserved.
 */

#ifndef STD_TYPES_H_FILE
#define STD_TYPES_H_FILE

/***********************************************************************************************************************
 * Includes
 **********************************************************************************************************************/

#ifdef __cplusplus
extern "C" {
#endif

/***********************************************************************************************************************
 * Definitions
 **********************************************************************************************************************/

#ifndef TRUE
#define TRUE (1U)
#endif

#ifndef FALSE
#define FALSE (0U)
#endif

#ifndef NULL_PTR
#define NULL_PTR ((void *) 0U)
#endif

#ifndef NULL
#define NULL NULL_PTR
#endif

typedef unsigned char      boolean;
typedef unsigned char      uint8;
typedef unsigned short     uint16;
typedef unsigned int       uint32;
typedef unsigned long long uint64;
typedef signed char        sint8;
typedef signed short       sint16;
typedef signed int         sint32;
typedef signed long long   sint64;
typedef float              float32;
typedef double             float64;

#ifdef __cplusplus
}
#endif

#endif /* STD_TYPES_H_FILE */
