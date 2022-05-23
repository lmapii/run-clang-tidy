
# version check for MAKE: using functions line 'file' and 'undefined', won't work with <= 3.81
_make_MIN := 4.0.0
$(if $(filter $(_make_MIN),$(firstword $(sort $(MAKE_VERSION) $(_make_MIN)))),,\
    $(error MAKE_VERSION must be at least $(_make_MIN)) )

# check that all required variables are defined
_make_VARS_ = \
    ROOT_PATH \
    MAKE_PATH

$(foreach var, $(_make_VARS_),\
    $(if $(filter undefined, $(origin $(var))),\
        $(error required variable $(var) is not defined)) )

# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #

# don't print 'Entering directory ...'
$(if $(findstring --no-print-directory,$(MAKEFLAGS)),,\
    $(eval MAKEFLAGS += --no-print-directory))

# optionally include override file if it exists at makefile level
-include $(CURDIR)/make_override.mak

# tooling, folders used by make.mak and helper functions
include $(MAKE_PATH)/make_tools.mak
include $(MAKE_PATH)/make_folders.mak

# silent :=
silent := @

define create_directories
$(foreach dir, $1,\
    $(info create/update directory $(dir)) \
    $(eval $(shell $(MKDIR) -p $(dir) )) )
endef

# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #

DEFS      ?=
INCS      ?=
OBJECTS    =

define create_rules_c
# p1 is the object file to create
# p2 lists all dependencies
$(eval _make_dep_BASE := $(patsubst %.o,%,$(notdir $1)))
$(if $(silent),,$(info .    create_rules_c for $1 (deps $2)))

# why wildcards are not supported: b/c clang, cmake etc. all use rules for cfile > ofile
# and not just the other way around. so there MUST be a .o file per .c file - which ain't really a restriction
$(eval $(call check_file_exists,$2,rule for $1))

# since includes are collected from all components we need a second expansion for that, i.e., it
# must only be evaluated once all components have been included and thus all paths are known
.SECONDEXPANSION:
$1: $2
$1: $2 $(BLD_PATH_DEP)/$(_make_dep_BASE).d
	$(silent)$(ECHO) -- compiling $(notdir $2)
	$(silent)$(CC) -MMD -MP -MF $(BLD_PATH_DEP)/$(_make_dep_BASE).Td $(C_FLAGS) $$(INCS) -c -o $1 $2
	$(silent)@$(MV) -f $(BLD_PATH_DEP)/$(_make_dep_BASE).Td $(BLD_PATH_DEP)/$(_make_dep_BASE).d && $(TOUCH) $1

# collect build rules for further processing, collect objects
CMD_LIST += $(realpath $(CURDIR))\n$(CC) $(C_FLAGS) $$(INCS) -c -o $1 $2\n$(strip $2)\n
OBJECTS  += $1
endef

# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #

# include dependencies
$(BLD_PATH_DEP)/%.d: ;
.PRECIOUS: $(BLD_PATH_DEP)/%.d

DEPS = $(wildcard $(patsubst %.o,$(BLD_PATH_DEP)/%.d,$(notdir $(OBJECTS))))

.PHONY: cmd-files
SKIP_GOALS += cmd-list

# create required build directories for non-skipped goals
ifeq (,$(filter $(SKIP_GOALS), $(MAKECMDGOALS)))
$(info build paths: $(BLD_PATHS))
$(eval $(call create_directories,$(BLD_PATHS)))
endif

cmd-files: force
	$(silent)$(RM) -rf $(BLD_PATH_CMD)""
	$(silent)$(MKDIR) -p $(BLD_PATH_CMD)""
	$(file >cmd-list.txt,$(CMD_LIST))
	$(file >cmd-incs.txt,$(CMD_INCS))
	$(silent)$(MV) cmd-list.txt $(BLD_PATH_CMD)
	$(silent)$(MV) cmd-incs.txt $(BLD_PATH_CMD)
	$(silent)$(ECHO) '.                                       '

build-data : cmd-files
	$(silent)$(PYTHON) $(MAKE_PATH)/parse_build.py \
		--root $(abspath $(CURDIR)) \
		--list $(BLD_PATH_CMD)/cmd-list.txt \
		--incs $(BLD_PATH_CMD)/cmd-incs.txt \
		--output $(BLD_ROOT)
	$(silent)$(MKDIR) -p $(abspath $(CURDIR))

force:
