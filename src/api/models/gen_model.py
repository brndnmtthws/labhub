#!/usr/bin/env python3

import click
from jinja2 import Environment, FileSystemLoader
env = Environment(
    loader=FileSystemLoader('templates'),
)


@click.command()
@click.argument('input_json', type=click.File('rb'))
@click.argument('output_rs', type=click.File('wb'))
@click.argument('mod_name')
@click.argument('model_name')
def inout(input_json, output_rs, mod_name, model_name):
    import json
    data = json.load(input_json)
    template = env.get_template('model.rs.j2')
    output_rs.write(bytearray(template.render(
        mod_name=mod_name, model_name=model_name, data=data), 'utf8'))
    print("Wrote to {}".format(output_rs.name))


if __name__ == '__main__':
    inout(None, None, None, None)
