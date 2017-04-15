#!/usr/bin/env python3

import dog_pb2
import person_pb2
import sys
import random
from google.protobuf.internal import encoder


def gen_person():
    person = person_pb2.Person(id=random.choice(range(0, 4)),
                               name=['raffi', 'khosrov', 'vahaken']
                               [random.choice(range(0, 3))])
    return person.SerializeToString()


def gen_dog():
    dog = dog_pb2.Dog(age=random.choice(range(0, 20)),
                      breed=['rottweiler', 'gsd', 'poodle']
                      [random.choice(range(0, 3))],
                      temperament=['chill', 'aggressive', 'excited']
                      [random.choice(range(0, 3))])
    return dog.SerializeToString()


def gen_null_dog():
    dog = dog_pb2.Dog(age=1337,
                      breed='st.bernard')
    return dog.SerializeToString()


if __name__ == '__main__':
    gen_funcs = [gen_person, gen_dog]
    if len(sys.argv) != 2:
        print('Usage: {0} stream|single|dirty'.format(sys.argv[0]))
        sys.exit(255)
    if sys.argv[1] == 'stream':
        while True:
            obj = gen_funcs[random.choice(range(0, 2))]()
            sys.stdout.buffer.write(encoder._VarintBytes(len(obj)) +
                                    obj)
    elif sys.argv[1] == 'single':
        sys.stdout.buffer.write(gen_funcs[random.choice(range(0, 2))]())
    elif sys.argv[1] == 'dirty':
        sys.stdout.buffer.write([b'1zk&', b'ksfU', b'^M^M']
                                [random.choice(range(0, 3))] +
                                gen_funcs[random.choice(range(0, 2))]() + b'\n')
    elif sys.argv[1] == 'null':
        sys.stdout.buffer.write(gen_null_dog())
