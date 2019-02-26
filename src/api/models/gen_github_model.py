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


def generate_structs(structs, struct_name, model):
    structs[struct_name] = []
    for key, value in model.items():
        print(key, value, type(value))
        if type(value) is dict:
            substruct_name = to_camel_case(key)
            structs = generate_structs(structs, substruct_name, value)
            structs[struct_name].append(
                "{}: Option<{}>,".format(key, substruct_name))
        # if type(value) is list:
        #     structs = generate_structs(structs, to_camel_case(key), value)
        elif value is None:
            structs[struct_name].append("r#{}: Option<String>,".format(key))
        elif type(value) is bool:
            structs[struct_name].append("r#{}: Option<bool>,".format(key))
        elif type(value) is str:
            structs[struct_name].append("r#{}: Option<String>,".format(key))
        elif type(value) is float:
            structs[struct_name].append("r#{}: Option<f64>,".format(key))
        elif type(value) is int:
            structs[struct_name].append("r#{}: Option<i64>,".format(key))
    return structs


@click.command()
@click.argument('input_json', type=click.File('rb'))
@click.argument('output_rs', type=click.File('wb'))
@click.argument('mod_name')
@click.argument('model_name')
def inout(input_json, output_rs, mod_name, model_name):
    import json
    data = json.load(input_json)
    merged_model = {}
    for event in data:
        merged_model = {**event, **merged_model}
    structs = generate_structs({}, 'WebhookEvent', merged_model)
    pprint(structs)
    template = env.get_template('model.rs.j2')
    output_rs.write(bytearray(template.render(
        mod_name=mod_name, model_name=model_name, structs=structs), 'utf8'))
    print("Wrote to {}".format(output_rs.name))


if __name__ == '__main__':
    inout(None, None, None, None)
