#!/usr/bin/env python3

from pprint import pprint
import click
from jinja2 import Environment, FileSystemLoader
env = Environment(
    loader=FileSystemLoader('templates'),
)


def to_camel_case(snake_str):
    components = snake_str.split('_')
    return ''.join(x.title() for x in components)


def get_type_for(name, value):
    if name == 'created_at' \
            or name == 'updated_at' \
        or name == 'pushed_at' \
            or name == 'due_on' \
            or name == 'closed_at':
        return "serde_json::value::Value"
    elif value is None:
        return "String"
    elif type(value) is int:
        return "i64"
    elif type(value) is bool:
        return "bool"
    elif type(value) is str:
        return "String"
    elif type(value) is float:
        return "f64"
    return "String"


def generate_type(key_name, value):
    return "pub {}: Option<{}>,".format(key_name, get_type_for(key_name, value))


def check_for_keywords(key):
    if key == 'ref' or key == 'type' or key == 'self':
        return '{}_key'.format(key)
    return key


def generate_structs(structs, struct_name, key, value, skip=False):
    key_name = check_for_keywords(key)

    if struct_name not in structs:
        structs[struct_name] = []

    if type(value) is not list and type(value) is not dict:
        obj = generate_type(key_name, value)
        if key_name != key:
            structs[struct_name].append('#[serde(rename = "{}")]'.format(key))
        structs[struct_name].append(obj)
    elif type(value) is list and len(value) > 0 and type(value[0]) is dict:
        substruct_name = struct_name + to_camel_case(key) + "Item"
        if struct_name.endswith("s"):
            substruct_name = struct_name[:-1] + to_camel_case(key) + "Item"
        for subkey, subvalue in value[0].items():
            structs = generate_structs(
                structs, substruct_name, subkey, subvalue)
        if key_name != key:
            structs[struct_name].append('#[serde(rename = "{}")]'.format(key))
        structs[struct_name].append(
            "pub {}: Option<Vec<{}>>,".format(key_name, substruct_name))
    elif type(value) is list and len(value) > 0:
        if key_name != key:
            structs[struct_name].append('#[serde(rename = "{}")]'.format(key))
        structs[struct_name].append("pub {}: Option<Vec<{}>>,".format(
            key_name, get_type_for(key_name, value[0])))
    elif type(value) is dict:
        substruct_name = struct_name + to_camel_case(key_name)
        for subkey, subvalue in value.items():
            structs = generate_structs(
                structs, substruct_name, subkey, subvalue)
        if not skip:
            if key_name != key:
                structs[struct_name].append(
                    '#[serde(rename = "{}")]'.format(key))
            structs[struct_name].append(
                "pub {}: Option<{}>,".format(key_name, substruct_name))

    return structs


@click.command()
@click.argument('input_json', type=click.File('rb'))
@click.argument('output_rs', type=click.File('wb'))
@click.argument('mod_name')
def inout(input_json, output_rs, mod_name):
    import json
    data = json.load(input_json)
    structs = {}
    for event_type, model in data.items():
        if type(model) is dict:
            for key, value in model.items():
                structs = generate_structs(
                    structs, to_camel_case(event_type), key, value)
        elif type(model) is list:
            structs = generate_structs(
                structs, to_camel_case(event_type), event_type, model[0], skip=True)
    template = env.get_template('model.rs.j2')
    output_rs.write(bytearray(template.render(
        mod_name=mod_name, structs=structs), 'utf8'))
    print("Wrote to {}".format(output_rs.name))


if __name__ == '__main__':
    # This is just to make the linter happy.
    inout(None, None, None, None)
