
# set default build path to project folder if it doesn't exist
$(if $(filter undefined, $(origin BLD_ROOT)),\
    $(if $(filter undefined, $(origin ROOT_PATH)),\
        $(error one of the variables BLD_ROOT or ROOT_PATH must be defined),\
            $(eval BLD_ROOT := $(ROOT_PATH)/_bld)) )

# where to create all the output
_make_BLD_PATH = $(BLD_ROOT)

# output directories
BLD_PATH_OBJ = $(_make_BLD_PATH)/obj
BLD_PATH_OUT = $(_make_BLD_PATH)/out
BLD_PATH_DEP = $(_make_BLD_PATH)/dep
BLD_PATH_CMD = $(_make_BLD_PATH)/cmd

# create list of output paths and remove duplicates
_make_BLD_PATHS =       \
    $(BLD_PATH_OUT)     \
    $(BLD_PATH_OBJ)     \
    $(BLD_PATH_DEP)     \
    $(BLD_PATH_CMD)

BLD_PATHS = $(sort $(_make_BLD_PATHS))
