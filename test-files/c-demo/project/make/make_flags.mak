
_COMMON_DEFS =          \
       -D SOME_DEFINE=1 \
       -g3              \
       -O0

# check C-standard (C90 or C99)
ifeq ($(C_STANDARD),C90)
_ISO_OPT_ = iso9899:1990
else
_ISO_OPT_ = gnu99
endif

# C compilation flags
_C_FLAGS := \
    -Wall                                       \
    -Wstrict-prototypes                         \
    -Wmissing-prototypes                        \
    -Werror-implicit-function-declaration       \
    -Wpointer-arith                             \
    -std=$(_ISO_OPT_)                           \
    -fno-strict-aliasing                        \
    -ffunction-sections                         \
    -fdata-sections

_C_FLAGS += \
    $(_COMMON_DEFS)
