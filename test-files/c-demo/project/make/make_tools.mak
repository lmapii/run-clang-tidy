
# support for commands like rm or mkdir on windows system via cygwin/mingw
# the variables MAKE_TOOLS_PATH, MAKE_CC are overridden in the project
# settings file (make_settings.mak)

ifeq ($(OS),Windows_NT)
CMDSEP := &
else
CMDSEP := ;
endif

ifneq ($(strip $(MAKE_TOOLS_PATH)),)
_MAKE_TOOLS_PATH = $(MAKE_TOOLS_PATH)/
endif

ECHO    ?= $(_MAKE_TOOLS_PATH)echo
ECHOESC ?= $(_MAKE_TOOLS_PATH)echo
RM      ?= $(_MAKE_TOOLS_PATH)rm
CP      ?= $(_MAKE_TOOLS_PATH)cp
MKDIR   ?= $(_MAKE_TOOLS_PATH)mkdir
MV      ?= $(_MAKE_TOOLS_PATH)mv
DATE    ?= $(_MAKE_TOOLS_PATH)date
SH      ?= $(_MAKE_TOOLS_PATH)sh
TEE     ?= $(_MAKE_TOOLS_PATH)tee
CAT     ?= $(_MAKE_TOOLS_PATH)cat
SED     ?= $(_MAKE_TOOLS_PATH)sed
TOUCH   ?= $(_MAKE_TOOLS_PATH)touch

PYTHON  ?= python3

# default path to cross compiler if not set via user settings
# MAKE_CC_PATH   ?= /opt/gcc-arm-none-eabi-7-2018-q2-update/bin
# MAKE_CC_PREFIX ?= arm-none-eabi-

_MAKE_CC_OVERRIDE :=
ifeq ($(strip $(MAKE_CC_PATH)),)
# no CC_PATH has been provided, meaning the user may want to use the CC/CXX/LD variables from $ENV
# check that the required variables are set, otherwise we'll fallback and use the CC_PATH settings
# removed CPP for parasoft compatibility
# CPP	:= $(_MAKE_CC_PREFIX)gcc -E
_make_Gxx_VARS_ = \
    CC            \
    CXX           \
    LD

$(foreach var, $(_make_Gxx_VARS_),\
    $(if $(filter undefined, $(origin $(var))),\
        $(eval _MAKE_CC_OVERRIDE := yes)) )
endif

# in case a make_override file is provided with a correct path some other build environment may
# still need its own settings, e.g., unit-test environments scanning the build procedure. this can
# be configured by using yet another flag for overring the configuration
CC_OVERRIDE ?=
ifneq ($(strip $(CC_OVERRIDE)),)
_MAKE_CC_OVERRIDE := yes
endif

ifeq ($(strip $(_MAKE_CC_OVERRIDE)),)
MAKE_CC_PATH   ?=
MAKE_CC_PREFIX ?=
ifneq ($(strip $(MAKE_CC_PATH)),)
    _MAKE_CC_PREFIX := $(MAKE_CC_PATH)/$(MAKE_CC_PREFIX)
else
    _MAKE_CC_PREFIX := $(MAKE_CC_PREFIX)
endif
CC  := $(_MAKE_CC_PREFIX)gcc
CXX := $(_MAKE_CC_PREFIX)g++
LD  := $(_MAKE_CC_PREFIX)gcc
AR  := $(_MAKE_CC_PREFIX)ar
else
${warning using override values for CC tools }
${info . CC:  $(CC)                          }
${info . CXX: $(CXX)                         }
${info . LD:  $(LD)                          }
${info . AR:  $(AR)                          }
endif
