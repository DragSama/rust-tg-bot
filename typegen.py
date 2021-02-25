import requests
import json
import re
import os
import glob

def camel_to_snake(name):
  name = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
  return re.sub('([a-z0-9])([A-Z])', r'\1_\2', name).lower()

def snake_to_camel(name):
    return ''.join(word.title() for word in name.split('_'))

if os.name == "nt":
    os.system("color")


def warn(text: str) -> None: # print red colored text
    print(f"\033[31m{text}\033[0m")


def success(text: str) -> None: # print green colored text
    print(f"\033[92m{text}\033[0m")


if not os.path.isdir("src/types"):
    os.mkdir("src/types")

methods = json.loads(
    re.sub(
        "Array of (\w+)",
        r"Vec<\1>",
        requests.get(
            "https://raw.githubusercontent.com/PaulSonOfLars/telegram-bot-api-spec/main/api.json"
        ).text,
    )
    .replace("Integer", "i64")
    .replace("Boolean", "bool")
    .replace("Float", "f64")
)["types"]
space = " " * 4

for method in methods:
    struct = f"#[derive(Debug, Serialize)]\npub struct {method}" + "{\n"
    method_data = methods[method]
    if "fields" in method_data:
        for field in method_data["fields"]:
            if field["required"]:
                struct += space + f'pub {field["name"]}: {field["types"][0]},\n'
            else:
                struct += space + f'pub {field["name"]}: Option<{field["types"][0]}>,\n'
        struct = struct[:-2] + "\n}"
        with open(f"types/{camel_to_snake(method)}.rs", "w") as file:
            file.write(struct)
        success(f"Generated file for {method}")
    else:
        warn(f"Failed to generate file for {method}: Missing 'field' key.")

files = [snake_to_camel(os.path.basename(x[:-3])) for x in glob.glob('src/types/*.rs')] # Get list of all files and convert them to CamelCase
rfiles = glob.glob('src/types/*rs') # List of all files

for x in rfiles:
    with open(x, 'r') as file:
        imports = "use crate::types::{"
        read = file.read()
        for i in files:
            if camel_to_snake(i) in (x):
              continue
            if i in read:
                imports += f'{camel_to_snake(i)}::{i}, '
        if imports == "use crate::types::{":
            warn(f'No imports found for {x}')
            continue
        imports = imports[:-2]
        imports += '}\n'
        with open(x, 'w') as f:
            f.write(imports+'\n'+read)
        success(f'Added imports to {x}')
        
