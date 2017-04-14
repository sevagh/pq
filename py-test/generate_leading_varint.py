#!/usr/bin/env python3

import leading_varint_pb2
import sys
import random


def gen_leading_varint():
    lv = leading_varint_pb2.LeadingVarint()
    lv.size = random.choice(range(0, 1503))
    return lv.SerializeToString()


if __name__ == '__main__':
    sys.stdout.buffer.write(gen_leading_varint())
