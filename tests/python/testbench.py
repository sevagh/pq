#!/usr/bin/env python3

import dog_pb2
import person_pb2
import sys
import random
from google.protobuf.internal import encoder


def single(varint=False):
    objs = [person_pb2.Person(id=random.choice(range(0, 4)),
                              name=['raffi', 'khosrov', 'vahaken']
                              [random.choice(range(0, 3))]),
            dog_pb2.Dog(age=random.choice(range(0, 20)),
                        breed=['rottweiler', 'gsd', 'poodle']
                        [random.choice(range(0, 3))],
                        temperament=['chill', 'aggressive', 'excited']
                        [random.choice(range(0, 3))])]
    obj = objs[random.choice(range(0, len(objs)))].SerializeToString()
    varint_ = encoder._VarintBytes(len(obj)) if varint else b''
    sys.stdout.buffer.write(varint_ + obj)


def stream(limit=10):
    for _ in range(0, limit):
        single(varint=True)


if __name__ == '__main__':
    eval(sys.argv[1])
