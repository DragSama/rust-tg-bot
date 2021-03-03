from pathlib import Path

import argparse
import requests
import json
import glob
import re
import os

# Thanks to https://stackoverflow.com/a/1176023
def camel_to_snake(name):
    name = re.sub("(.)([A-Z][a-z]+)", r"\1_\2", name)
    return re.sub("([a-z0-9])([A-Z])", r"\1_\2", name).lower()


def snake_to_camel(name):
    return "".join(word.title() for word in name.split("_"))


class TypeGen:
    def __init__(self, path, replace, verbose):
        if path.endswith("/") or path.endswith("\\"):
            path = path[:-1]
        self.path = Path(path) / "types"
        if not os.path.isdir(self.path):
            os.mkdir(self.path)
        self.types = json.loads(
            re.sub(
                "Array of (\w+)",
                r"Vec<\1>",
                requests.get(
                    "https://raw.githubusercontent.com/PaulSonOfLars/telegram-bot-api-spec/main/api.json"
                ).text,
            )
            .replace("Integer", "i32")
            .replace("Boolean", "bool")
            .replace("Float", "f64")
        )["types"]
        self.replace = replace
        self.space = " " * 4
        self.files = []
        self.verbose = verbose

        self.save_struct()
        self.save_mod()
        self.add_imports()

    def verbose_print(self, data: str):
        """Only print if verbose is set to True"""
        if self.verbose:
            print("[TYPES]", data)

    def save_struct(self):
        count = 0
        empty_count = 0
        for type in self.types:
            struct = f"use serde::Deserialize;\n\n#[derive(Debug, Deserialize)]\npub struct {type}" + "{\n"
            type_data = self.types[type]
            file_path = self.path / f"{camel_to_snake(type)}.rs"
            if os.path.exists(file_path) and not self.replace:
                self.verbose_print(
                    f"Ignoring {str(file_path)} because already exists.")
                continue
            if "fields" in type_data:
                for field in type_data["fields"]:
                    fieldtype = field["types"][0]
                    if "64 bit integer" in field["description"].lower() and type == "i32":
                        fieldtype = "i64"
                    if fieldtype == type:
                        fieldtype = f"Box<{type}>"
                    if field["required"]:
                        struct += (
                            self.space +
                            f'pub {field["name"]}: {fieldtype},\n'
                        )
                    else:
                        struct += (
                            self.space
                            + f'pub {field["name"]}: Option<{fieldtype}>,\n'
                        )
                struct = struct[:-2] + "\n}"
                self.files.append(camel_to_snake(type))
                with open(file_path, "w") as file:
                    file.write(struct)
                self.verbose_print(f"Generated file for {type}")
                count += 1
            else:
                struct = struct[:-2] + ";\n"
                with open(file_path, "w") as file:
                    file.write(struct)
                self.verbose_print(
                    f"Generating empty struct for {type}| Reason: Missing 'field' key."
                )
                empty_count += 1
        print(
            "[TYPES]", f"Generated {count} struct and {empty_count} empty structs.")

    def save_mod(self):
        text = ""
        files = self.files
        for file in files:
            text += f"mod {file};\n"
        text += "\n\n"
        for file in files:
            text += f"pub use {file}::{snake_to_camel(file)};\n"
        mod_file = self.path / "mod.rs"
        with open(mod_file, "w") as f:
            f.write(text)
        self.verbose_print(f"Created mod.rs file at {str(mod_file)}.")

    def add_imports(self):
        # Get list of all files and convert them to CamelCase
        files = [
            snake_to_camel(os.path.basename(x[:-3]))
            for x in glob.glob(os.path.join(str(self.path), "*.rs"))
        ]
        # List of all files
        rfiles = glob.glob(os.path.join(str(self.path), "*.rs"))
        no_imports = 0
        with_imports = 0
        for x in rfiles:
            if "mod.rs" in x:
                continue
            with open(x, "r") as file:
                imports = "use crate::types::{"
                read = file.read()
                for i in files: # Check all files available in /types to see if they are imported in this file
                    # don't import self, though this can cause problems
                    # i.e. location is in chat_location
                    # Feel free to pr If you know a better way to avoid the above problem from happening
                    # and still check if file is not same
                    if camel_to_snake(i) in x:
                        continue
                    if i in read:
                        imports += f"{i}, "
                if imports == "use crate::types::{":
                    self.verbose_print(f"No imports found for {x}")
                    no_imports += 1
                    continue
                imports = imports[:-2]
                imports += "};\n"
                with open(x, "w") as f:
                    f.write(imports + "\n" + read)
                self.verbose_print(f"Added imports to {x}")
                with_imports += 1
        print(
            "[TYPES]",
            f"Added imports to {with_imports} files, Couldn't find any imports for {no_imports} files.",
        )

class MethodGen:
    def __init__(self, path, replace, verbose):
        if path.endswith("/") or path.endswith("\\"):
            path = path[:-1]
        self.path = Path(path) / "methods"
        if not os.path.isdir(self.path):
            os.mkdir(self.path)
        self.methods = json.loads(
            re.sub(
                "Array of (\w+)",
                r"Vec<\1>",
                requests.get(
                    "https://raw.githubusercontent.com/PaulSonOfLars/telegram-bot-api-spec/main/api.json"
                ).text,
            )
            .replace("Integer", "i32")
            .replace("Boolean", "bool")
            .replace("Float", "f64")
        )["methods"]
        self.replace = replace
        self.space = " " * 4
        self.files = []
        self.verbose = verbose

        self.save_struct()
        self.save_mod()

    def verbose_print(self, data: str):
        """Only print if verbose is set to True"""
        if self.verbose:
            print("[METHODS]", data)

    def save_struct(self):
        files = self.files
        methods = self.methods
        count = 0
        empty_count = 0
        for method in methods:
            file_path = self.path / f"{camel_to_snake(method)}.rs"
            if os.path.exists(file_path) and not self.replace:
                self.verbose_print(
                    f"Ignoring {str(file_path)} because already exists."
                )
                continue
            struct = f"use serde::Serialize;\n\n\n#[derive(Debug, Serialize)]\npub struct {method}" + "{\n"
            method_data = methods[method]
            if "fields" in method_data:
                for field in method_data["fields"]:
                    type = field["types"][0]
                    if "64 bit integer" in field["description"].lower() and type == "i32":
                        type = "i64"
                    if field["required"]:
                        struct += self.space + f'pub {field["name"]}: {type},\n'
                    else:
                        struct += self.space + f'pub {field["name"]}: Option<{type}>,\n'
                struct = struct[:-2] + "\n}"
                files.append(camel_to_snake(method))
                with open(file_path, "w") as file:
                    file.write(struct)
                count += 1
                self.verbose_print(f"Generated file for {method}")
            else:
                struct = struct[:-2] + ';'
                with open(file_path, "w") as file:
                    file.write(struct)
                empty_count += 1
                self.verbose_print(
                    f"Generating empty struct for {method}| Reason: Missing 'fields' key."
                )
        print("[METHODS]", f"Generated {count} struct and {empty_count} empty structs.")

    def save_mod(self):
        text = ""
        files = self.files
        for file in files:
            text += f"mod {file};\n"
        text += "\n\n"
        for file in files:
            text += f"pub use {file}::{snake_to_camel(file)};\n"
        mod_file = self.path / "mod.rs"
        with open(mod_file, "w") as f:
            f.write(text)
        self.verbose_print(f"Created mod.rs file at {str(mod_file)}.")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Auto generate types/methods.")
    parser.add_argument(
        "--types", "--type", help="Generate types", action="store_true", default=False
    )
    parser.add_argument(
        "--methods",
        "--method",
        help="Generate methods",
        action="store_true",
        default=False,
    )
    parser.add_argument(
        "--path", help="Path to save methods/types.", type=str, default="src/"
    )
    parser.add_argument(
        "--replace",
        help="Replace older files with updated version.",
        default=False,
        action="store_true",
    )
    parser.add_argument(
        "--verbose", "--v", help="Print more info.", default=False, action="store_true"
    )
    args = parser.parse_args()
    if not args.types and not args.methods:
        parser.error("You need to specify atleast one of --types, --methods.")
    if args.types:
        TypeGen(args.path, args.replace, args.verbose)
    if args.methods:
        MethodGen(args.path, args.replace, args.verbose)
