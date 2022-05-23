# -*- coding: utf-8 -*-
#
# Copyright (c) Martin Lampacher. All rights reserved.
#
# Purpose
#    script to generate JSON Compile Database and build-data for further clang-tool processing
#    https://clang.llvm.org/docs/JSONCompilationDatabase.html
# --

import json
import argparse
import logging
import os


def __extract_from_list(dump, collection):
    # structure of list-file: path, command, file
    _read_list_path = 0
    _read_list_step = 1
    _read_list_file = 2

    ccd_entry = None
    step = _read_list_path

    for line in dump.split(r"\n"):
        str_ = line.strip()
        # logging.debug('extracting list "%s"', str_)
        if str_ == "":
            continue
        elif step is _read_list_path:
            ccd_entry = {"directory": str_}
            step = _read_list_step
        elif step is _read_list_step:
            ccd_entry["command"] = str_  # pylint: disable=E1137
            step = _read_list_file
        elif step is _read_list_file:
            ccd_entry["file"] = str_  # pylint: disable=E1137
            collection["commands"].append(ccd_entry)
            collection["files"].append(str_)
            step = _read_list_path
        else:
            raise ValueError("unknown step %s" % step)


def __extract_from_incs(dump, collection):
    for line in dump.split(r"\n"):
        str_ = line.strip()
        if not str_:
            continue
        logging.debug('extracting include "%s"', str_)

        for inc in str_.split(" "):
            collection["includes"].append(inc.strip())
            logging.debug('listdir for %s', inc)
            try:
                paths = sorted(os.listdir(inc.strip()))
            except FileNotFoundError:
                logging.debug('  skipping %s, FileNotFound', inc.strip())
                break

            for file_ in paths:
                ext = str.lower(os.path.splitext(file_)[1])
                if ext != ".h":
                    continue
                # collection["headers"].append(os.path.join(inc.strip(), file_))
                # os.path.join uses '\\' for windows files, which simply won't work :(
                collection["headers"].append("%s/%s" % (inc.strip(), file_))


def __execute__(root, in_incs, in_list, outpath):
    dump_list = ""
    with open(in_list, "r", encoding="utf-8") as file_:
        dump_list = file_.read()

    dump_incs = ""
    with open(in_incs, "r", encoding="utf-8") as file_:
        dump_incs = file_.read()

    collection = {"root": root, "commands": [], "headers": [], "includes": [], "files": []}
    __extract_from_list(dump_list, collection)
    __extract_from_incs(dump_incs, collection)

    # logging.info(json.dumps(collection, indent=2))

    if not os.path.exists(outpath):
        os.makedirs(outpath)

    with open(os.path.join(outpath, "compile_commands.json"), "w", encoding="utf-8") as handle_:
        handle_.write(json.dumps(collection["commands"], indent=2))

    with open(os.path.join(outpath, "build.json"), "w", encoding="utf-8") as handle_:
        handle_.write(json.dumps(collection, indent=2))


# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -


def __is_valid_file__(parser_, arg):
    if not os.path.isfile(arg):
        parser_.error("'%s' not found / not a file." % arg)
        return None
    return arg


def __is_valid_folder__(parser, arg):
    if not os.path.exists(arg):
        os.mkdir(arg)

    if not os.path.isdir(arg):
        parser.error("'%s' not found / not a folder." % arg)
        return None
    return arg


if __name__ == "__main__":

    logging.basicConfig(
        level=logging.INFO,
        format='. %(message)s',
        datefmt='(%H:%M:%S)')

    logging.info("")
    logging.info("executing %s ...", os.path.basename(__file__))

    PARSER_ = argparse.ArgumentParser(
        description="parse and modify hex files")

    PARSER_.add_argument(
        '--list',
        dest="input_list",
        required=True,
        metavar="input-list",
        type=lambda x: __is_valid_file__(PARSER_, x))

    PARSER_.add_argument(
        '--incs',
        dest="input_incs",
        required=True,
        metavar="input-incs",
        type=lambda x: __is_valid_file__(PARSER_, x))

    PARSER_.add_argument(
        '--root',
        dest="project_root",
        required=True,
        metavar="project-root",
        type=lambda x: __is_valid_folder__(PARSER_, x),
        help="")

    PARSER_.add_argument(
        '-o',
        '--output',
        dest="output_path",
        required=True,
        metavar="output directory",
        type=lambda x: __is_valid_folder__(PARSER_, x))

    # not adding an argument to restrict the packages since the compile database is really
    # required over the whole project

    ARGS_ = PARSER_.parse_args()
    __execute__(ARGS_.project_root, ARGS_.input_incs, ARGS_.input_list, ARGS_.output_path)
