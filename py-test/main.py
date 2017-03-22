#!/usr/bin/env python3

import addressbook_pb2
import random
import string
import sys


def gen_person():
    person = addressbook_pb2.Person()
    person.id = random.choice(range(0, 4))
    person.name = ''.join(random.choice(string.ascii_uppercase + string.digits)
                          for _ in range(12))
    return person


if __name__ == '__main__':
    person = gen_person()
    sys.stdout.buffer.write(person.SerializeToString())
