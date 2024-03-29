#
# demo makefile
#

PRJ_NAME := build-demo
ROOT_PATH := ..
MAKE_PATH := $(ROOT_PATH)/project/make

CC_OVERRIDE := yes
ifeq ($(OS),Windows_NT)
CC :=$(ROOT_PATH)/../../artifacts/clang/clang.exe
else
CC :=$(ROOT_PATH)/../../artifacts/clang/clang
endif
# CC?=clang

# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #

skip_goals = clean cleanout cleanall

include $(MAKE_PATH)/make_flags.mak
include $(MAKE_PATH)/make.mak

src_paths := \
    $(ROOT_PATH)/pkg_a/module_a \
    $(ROOT_PATH)/pkg_b/module_b \
    $(ROOT_PATH)/pkg_b/module_c \
    $(ROOT_PATH)/pkg_b/module_fix \
    $(ROOT_PATH)/subfolder/pkg_c/module_sub_a \
    $(ROOT_PATH)/subfolder/pkg_c/module_sub_b \
    $(ROOT_PATH)/subfolder/pkg_c/module_untidy \
    $(ROOT_PATH)/project/src

inc_dirs = $(foreach inc_path, $(sort $(src_paths)), -I $(inc_path))
c_files  = $(foreach src_path, $(src_paths), $(wildcard $(src_path)/*.c))


C_FLAGS := $(_C_FLAGS)
INCS := $(inc_dirs)

$(foreach src, $(c_files),\
    $(eval $(call create_rules_c,\
    $(patsubst %$(suffix $(src)),$(BLD_PATH_OBJ)/%.o,$(notdir $(src))),$(src))) )

# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #

.DEFAULT_GOAL := all
.PHONY: all prebuild postbuild clean

# full build process
all : prebuild $(OBJECTS) postbuild

prebuild :
	$(silent)$(ECHO) '. building $(PRJ_NAME)  '

postbuild : prebuild
	$(silent)$(ECHO) '.                                       '
	$(silent)$(ECHO) '. make done for $(PRJ_NAME).            '
	$(silent)$(ECHO) '.                                       '

clean:
	$(silent)$(ECHO) "### removing directories $(BLD_ROOT)"
	$(silent)$(RM) -rf $(BLD_ROOT)
