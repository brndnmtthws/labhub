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
    if name == 'created_at' or name == 'updated_at' or name == 'pushed_at':
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


def generate_structs_inner(structs, struct_name, key, value):
    key_name = key
    if key == 'ref' or key == 'type':
        key_name = 'r#{}'.format(key_name)
    if type(value) is dict:
        substruct_name = to_camel_case(key_name)
        structs = generate_structs(
            structs, struct_name + substruct_name, value)
        structs[struct_name].append(
            "pub {}: Option<{}>,".format(key_name, struct_name + substruct_name))
    elif type(value) is list:
        if len(value) == 0:
            # skip empty lists, don't know the type
            return
        substruct_name = to_camel_case(key_name)
        if substruct_name[-1] == 's':
            substruct_name = substruct_name[:-1]
        if type(value[0]) == dict:
            structs[struct_name + substruct_name] = []
            structs = generate_structs_inner(structs, struct_name + substruct_name,
                                             substruct_name, value[0])
            structs[struct_name +
                    substruct_name] = structs[struct_name + substruct_name][:-1]
            structs[struct_name].append(
                "pub {}: Option<Vec<{}>>,".format(key_name, struct_name + substruct_name))
        else:
            subtype = get_type_for(struct_name, value[0])
            structs[struct_name].append(
                "pub {}: Option<Vec<{}>>,".format(key_name, subtype))
    else:
        obj = generate_type(key_name, value)
        structs[struct_name].append(obj)
    return structs


def generate_structs(structs, struct_name, model):
    if struct_name in structs.keys():
        print("'{}' already in struct (duplicate type?)".format(struct_name))
    structs[struct_name] = []
    for key, value in model.items():
        generate_structs_inner(structs, struct_name, key, value)
    return structs


@click.command()
@click.argument('input_json', type=click.File('rb'))
@click.argument('output_rs', type=click.File('wb'))
@click.argument('mod_name')
@click.argument('model_name')
def inout(input_json, output_rs, mod_name, model_name):
    import json
    data = json.load(input_json)
    structs = {}
    for event_type, model in data.items():
        structs = generate_structs(structs, event_type.title(), model)
    template = env.get_template('model.rs.j2')
    output_rs.write(bytearray(template.render(
        mod_name=mod_name, model_name=model_name, structs=structs), 'utf8'))
    print("Wrote to {}".format(output_rs.name))


if __name__ == '__main__':
    inout(None, None, None, None)
