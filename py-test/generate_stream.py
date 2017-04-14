#!/usr/bin/env python3

import leading_varint_pb2
import dog_pb2
import person_pb2
import sys
import random


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
    temperaments = ['chill', 'aggressive', 'excited']
    dog.breed = breeds[random.choice(range(0, 3))]
    dog.temperament = temperaments[random.choice(range(0, 3))]
    return dog


def gen_leading_varint(size):
    lv = leading_varint_pb2.LeadingVarint()
    lv.size = size
    return lv.SerializeToString()


if __name__ == '__main__':
    gen_funcs = [gen_person, gen_dog]
    while True:
        obj = gen_funcs[random.choice(range(0, 2))]().SerializeToString()
        sys.stdout.buffer.write(gen_leading_varint(len(obj)))
        sys.stdout.buffer.write(obj)
