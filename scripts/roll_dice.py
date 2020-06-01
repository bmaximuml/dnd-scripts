#!/usr/bin/python3

import click
import random

from tabulate import tabulate


@click.command()
@click.option("--dice", "-d", default=20, show_default=True, help="Which dice to roll")
@click.option("--count", "-c", default=1, show_default=True, help="How many dice to roll")
@click.option("--modifier", "-m", default=0, show_default=True, help="What modifier to add")
def roll_dice(dice, count, modifier):
    headers = ['', 'Modified', 'Original']

    random.seed()
    rolls = []
    for i in range(count):
        roll = random.randint(1, dice)
        rolls.append([i + 1, roll + modifier, roll])

    click.echo(tabulate(rolls, headers=headers, tablefmt="pretty", colalign=("right", "right", "right")))
    click.echo(f'Total: {str(sum([roll[1] for roll in rolls]))} ({str(sum([roll[2] for roll in rolls]))})')


if __name__ == "__main__":
    roll_dice()
