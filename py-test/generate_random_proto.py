#!/usr/bin/env python3

import dog_pb2
import person_pb2
import random
import sys


def gen_person():
    person = person_pb2.Person()
    person.id = random.choice(range(0, 4))
    names = ['raffi', 'khosrov', 'vahaken']
    person.name = names[random.choice(range(0, 3))]
    return person


def gen_dog():
    dog = dog_pb2.Dog()
    dog.age = random.choice(range(0, 20))
    breeds = ['rottweiler', 'gsd', 'poodle']
    dog.breed = breeds[random.choice(range(0, 3))]
    return dog


if __name__ == '__main__':
    gen_funcs = [gen_person, gen_dog]
    sys.stdout.buffer.write(
        gen_funcs[random.choice(range(0, 2))]().
        SerializeToString())
