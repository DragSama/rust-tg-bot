# ###########################################################
# ###########################################################
#         Highly unoptimized and untested code below 
#         Edit, View or run at your own risk kthx
# ###########################################################
# ###########################################################

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

def convert(text):
    text = text.replace("Integer", "i32").replace("Boolean", "bool").replace("Float", "f64")
    text = re.sub(
        "Array of Array of (\w+)",
        r"Vec<Vec<\1>>",
        text,
    ) # Handle [[InlineKeyboardButton]]
    text = re.sub(
        "Array of (\w+)",
        r"Vec<\1>",
        text,
    )
    return text


# I don't recommend running this ever, In worse case scenario it will fill whole types folder with random stuff (also https://imgur.com/4GptC5V) 
class TypeGen:
    def __init__(self, path, replace, verbose, to_ignore):
        if path.endswith("/") or path.endswith("\\"):
            path = path[:-1]
        self.path = Path(path) / "types"
        if not os.path.isdir(self.path):
            os.mkdir(self.path)
        self.types = json.loads(convert(requests.get("https://raw.githubusercontent.com/PaulSonOfLars/telegram-bot-api-spec/main/api.json").text))["types"]
        self.replace = replace
        self.space = " " * 4
        self.files = []
        self.verbose = verbose
        self.to_ignore = to_ignore
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
            struct = "use serde::{Deserialize, Serialize};\n\n" + f"#[derive(Debug, Deserialize, Serialize, Clone)]\npub struct {type}" + "{\n"
            type_data = self.types[type]
            if (file := camel_to_snake(type)) + '.rs' in self.to_ignore:
                self.verbose_print(f'Ignoring {file}.')
                return
            print(file)
            file_path = self.path / f"{file}.rs"
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
                    name = field["name"]
                    if name == "type":
                        name = "r#type"
                    if name == "mod":
                        name = "r#mod"
                    desc = f"{self.space}/// {field['description'].encode('ascii', 'ignore').decode('ascii')}\n"
                    if field["required"]:
                        struct += (
                            desc +
                            self.space +
                            f'pub {name}: {fieldtype},\n'
                        )
                    else:
                        struct += (
                            desc +
                            self.space
                            + f'pub {name}: Option<{fieldtype}>,\n'
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
        files = glob.glob(os.path.join(str(self.path), "*.rs"))
        for file in files:
            if "mod.rs" in file:
                continue
            text += f"mod {os.path.basename(file)[:-3]};\n"
        text += "\n\n"
        for file in files:
            if "mod.rs" in file:
                continue
            text += f"pub use {os.path.basename(file)[:-3]}::{snake_to_camel(os.path.basename(file)[:-3])};\n"
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
            # print(len(list(set(rfiles))), len(rfiles))
            with open(x, "r") as file:
                imports = "use crate::types::{"
                read = file.read()
                for i in files: # Check all files available in /types to see if they are imported in this file.
                    if camel_to_snake(i) == os.path.basename(x[:-3]): # Don't use self.
                        continue
                    if 'mod' in i:
                        continue
                    if i in read:
                        imports += f"{i}, "
                        # print(i)
                if imports == "use crate::types::{":
                    self.verbose_print(f"No imports found for {x}")
                    no_imports += 1
                    continue
                imports = imports[:-2]
                imports += "};\n"
                # print(imports)
            with open(x, "w") as f:
                f.write(imports + "\n" + read)
            # print(imports + "\n" + read)
            self.verbose_print(f"Added imports to {x}")
            with_imports += 1
        print(
            "[TYPES]",
            f"Added imports to {with_imports} files, Couldn't find any imports for {no_imports} files.",
        )

class MethodGen:
    def __init__(self, path, replace, verbose, to_ignore):
        if path.endswith("/") or path.endswith("\\"):
            path = path[:-1]
        self.path = Path(path) / "methods"
        self.types_path = Path(path) / "types"
        if not os.path.isdir(self.path):
            os.mkdir(self.path)
        self.to_ignore = to_ignore
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
        if not os.path.isdir(self.types_path):
            print("[METHODS] Types folder doesn't exist! Skipping adding imports..")
        else:
            self.add_imports()

    # I know there are several ways to improve this function (whole code tbh)
    # but at the time i was writing this i just wanted to make it works.
    def gen_impl(self, method_data):
        name = method_data['name'][0].upper() + method_data['name'][1:]
        method_name = method_data['name']
        impl = f"impl<'a> {name}<'a> " + "{\n"
        impl += f"{self.space}pub fn new(bot: &'a Bot, "
        none_values = []
        with_values = []
        for field in method_data['fields']:
            type = field["types"][0]
            if "64 bit integer" in field["description"].lower() and type == "i32":
                type = "i64"
            name = field["name"]
            if name == "type":
                name = "r#type"
            if field['required']:
                impl += f"{name}: {type}, "
                with_values.append(name)
            else:
                none_values.append(name)
        impl += ") -> Self {\n" + self.space*2 + "Self {\n" 
        for with_value in with_values:
            impl += f"{self.space*3}{with_value}: {with_value},\n"
        for none_value in none_values:
            impl += f"{self.space*3}{none_value}: None,\n"
        impl += f"{self.space*3}bot: bot,\n"
        impl += self.space*2 + "}\n" + self.space + "}\n"
        return_type = convert(method_data['returns'][0])
        impl += f"{self.space}pub async fn send(self) -> Result<{return_type}> " + "{\n"
        # impl += f'{self.space*2}Ok(serde_json::from_str::<{convert(method_data["returns"][0])}>()?)\n' + self.space + '}\n'
        impl += f'{self.space*2}let string = serde_json::to_string(&self)?;\n'
        impl += f'{self.space*2}let resp = self.bot.send("{method_name}", Some(string)).await?;\n'
        impl += f'{self.space*2}Ok(serde_json::from_str::<{return_type}>(&resp.text().await?)?)\n' + self.space + '}\n'
        for field in method_data['fields']:
            type = field["types"][0]
            if "64 bit integer" in field["description"].lower() and type == "i32":
                type = "i64"
            name = field["name"]
            if name == "type":
                name = "r#type"
            impl += f"{self.space}pub fn {name}(mut self, {name}: {type}) -> Self " + "{\n" + self.space*2
            if field["required"]:
                impl += f"self.{name} = {name};\n{self.space*2}self\n" + self.space +"}\n"
            else:
                impl += f"self.{name} = Some({name});\n{self.space*2}self\n" + self.space +"}\n"
        impl += '\n}'
        return impl

    def verbose_print(self, data: str):
        """Only print if verbose is set to True"""
        if self.verbose:
            print("[METHODS]", data)

    def save_struct(self):
        files = self.files
        methods = self.methods
        count = 0
        empty_count = 0
        empty_files = []
        for method in methods:
            if (file := camel_to_snake(method)) + '.rs' in self.to_ignore:
                self.verbose_print(f'Ignoring {file}.')
                continue
            file_path = self.path / f"{file}.rs"
            if os.path.exists(file_path) and not self.replace:
                self.verbose_print(
                    f"Ignoring {str(file_path)} because already exists."
                )
                continue
            struct = "use crate::{bot::Bot, error::Result};\n" + f'use serde::Serialize;\nuse serde_json;\n\n\n#[must_use = "{method[0].upper() + method[1:]} does nothing until you `send` it"]\n' + f"#[derive(Serialize)]\npub struct {method[0].upper() + method[1:]}<'a>" + "{\n"
            method_data = methods[method]
            if "fields" in method_data:
                struct += f"{self.space}#[serde(skip)]\n{self.space}bot: &'a Bot,\n"
                for field in method_data["fields"]:
                    type = field["types"][0]
                    if "64 bit integer" in field["description"].lower() and type == "i32":
                        type = "i64"
                    desc = f"{self.space}/// {field['description'].encode('ascii', 'ignore').decode('ascii')}\n"
                    name = field["name"]
                    if name == "type":
                        name = "r#type"
                    if field["required"]:
                        struct += desc + self.space + f'pub {name}: {type},\n'
                    else:
                        struct += desc + self.space + f'#[serde(skip_serializing_if = "Option::is_none")]\n{self.space}pub {name}: Option<{type}>,\n'
                struct = struct[:-2] + "\n}"
                struct += '\n\n' + self.gen_impl(method_data)
                files.append(camel_to_snake(method))
                with open(file_path, "w") as file:
                    file.write(struct)
                count += 1
                self.verbose_print(f"Generated file for {method}")
            else:
                struct = struct[:-2] + ';'
                empty_count += 1
                empty_files.append(file_path)
                self.verbose_print(
                    f"Generating empty struct for {method}| Reason: Missing 'fields' key."
                )
        print("[METHODS]", f"Generated {count} struct and {empty_count} empty structs ({empty_files}).")

    def save_mod(self):
        text = ""
        files = self.files
        for file in files:
            text += f"mod {file};\n"
        text += "\n\n"
        for file in files:
            struct_name = snake_to_camel(file)
            text += f"pub use {file}::{struct_name[0].upper() + struct_name[1:]};\n"
        mod_file = self.path / "mod.rs"
        with open(mod_file, "w") as f:
            f.write(text)
        self.verbose_print(f"Created mod.rs file at {str(mod_file)}.")

    def add_imports(self):
        count = 0
        no_import = 0
        for method_file in glob.glob(os.path.join(str(self.path), "*.rs")):
            imports = "use crate::types::{"
            if method_file in self.to_ignore:
                self.verbose_print(
                    f"Ignoring {method_file}."
                )
                continue
            with open(method_file, 'r') as file:
                read = file.read()
                for file in glob.glob(os.path.join(str(self.types_path), "*.rs")):
                    if 'mod' in file:
                        continue
                    basename = snake_to_camel(os.path.basename(file)[:-3])
                    if basename in read:
                        imports += basename + ', '
            if imports == "use crate::types::{":
                self.verbose_print(f"No imports found for {method_file}")
                no_import += 1
                continue
            imports = imports[:-2] + '};'
            with open(method_file, 'w') as f:
                f.write(imports + '\n' + read)
            self.verbose_print(f'Added imports for {method_file}')
            count += 1
        print(f"[METHODS] Added imports to {count} files, Couldn't find imports for {no_import} files.")





if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Auto generate types/methods.")
    parser.add_argument(
        "--types", "-t", help="Generate types", action="store_true", default=False
    )
    parser.add_argument(
        "--methods",
        "-m",
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
        "-verbose", "-v", help="Print more info.", default=False, action="store_true"
    )
    parser.add_argument(
        "-ignore", "-i", help="ignore X files", default="", nargs="*"
    )
    args = parser.parse_args()
    if not args.types and not args.methods:
        parser.error("You need to specify atleast one of --types, --methods.")
    if args.types:
        TypeGen(args.path, args.replace, args.verbose, args.ignore)
    if args.methods:
        MethodGen(args.path, args.replace, args.verbose, args.ignore)
    print('\nNote: Use --verbose for more info.')
