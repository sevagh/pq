#!/usr/bin/env python3

import dog_pb2
import person_pb2
import cat_pb2
import sys
import random
from google.protobuf.internal import encoder
from optparse import OptionParser


def single(msgtype, stream=False):
    if msgtype == 'cat':
        obj = cat_pb2.Cat(is_lazy=bool(random.getrandbits(1)))
    elif msgtype == 'dog':
        obj = dog_pb2.Dog(age=random.choice(range(0, 20)),
                          breed=['rottweiler', 'gsd', 'poodle']
                          [random.choice(range(0, 3))],
                          temperament=['chill', 'aggressive', 'excited']
                          [random.choice(range(0, 3))])
    elif msgtype == 'person':
        obj = person_pb2.Person(id=random.choice(range(0, 4)),
                                name=['raffi', 'khosrov', 'vahaken']
                                [random.choice(range(0, 3))])
    else:
        usage()
    obj = obj.SerializeToString()
    varint_ = encoder._VarintBytes(len(obj)) if stream else b''
    sys.stdout.buffer.write(varint_ + obj)


def stream(msgtype, limit):
    for _ in range(0, limit):
        single(msgtype, stream=True)


def usage():
    raise ValueError(('Usage: {0} <single|stream> <dog|person|cat>'
                      ' [--count c]').format(sys.argv[0]))


if __name__ == '__main__':
    parser = OptionParser()
    parser.add_option("--count", dest="count", help="stream count",
                      metavar="COUNT")
    (options, args) = parser.parse_args(args=sys.argv)
    if len(args) != 3:
        usage()

    if args[1] == 'single':
        single(args[2])
    elif args[2] == 'stream':
        stream(args[2], options.count)
