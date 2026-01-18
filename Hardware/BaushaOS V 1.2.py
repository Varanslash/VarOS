# Boot Code
# Every Import Known To Man

import machine
import sys
import gc
import math
import cmath
import array
import struct
import json
import re
import io
import collections
import heapq
import binascii
import hashlib
import random
import time
import select
import errno
import micropython
import uctypes
import ubinascii
import uheapq
import uio
import ujson
import uos
import ure
import uselect
import ustruct
import utime
import rp2
import _rp2
import _thread
import uasyncio
import uasyncio.core
import uasyncio.event
import uasyncio.funcs
import uasyncio.lock
import uasyncio.stream
from machine import Pin, ADC, PWM, I2C, SPI, UART, Timer, RTC, WDT
from micropython import const, opt_level, alloc_emergency_exception_buf
from machine import mem8, mem16, mem32  # Direct memory access
from array import array
from collections import namedtuple, OrderedDict, deque

filesystem = {

}

apps = {

}

drivers = {

}

def startup():
    try:
        with open("config.bau", "r") as f:
            data = f.read().split("\n")
            global filesystem
            global apps
            global drivers
            filesystem = json.loads(data[0])
            apps = json.loads(data[1])
            drivers = json.loads(data[2])
    except FileNotFoundError:
        pass

startup()

errorcodes = [
    "ERR: A catastrophic and unrecoverable error has occured. Please contact the current owners and developers on GitHub to troubleshoot this error.", # 0
    "ERR: Empty Command", # 1
    "ERR: Item does not exist" # 2
]

python3 = 0
txtedit = 0
appmake = 0
drivers = 0

def errorcode(code):
    print(errorcodes[code])
    time.sleep(1)

def exit():
    with open("config.bau", "w") as f:
        f.write(json.dumps(filesystem) + "\n" + json.dumps(apps) + "\n" + json.dumps(drivers))

# Failure handler in the event of app or driver failure
def BSOD():
    UART.deinit()
    Pin.off()
    Pin.init(Pin.IN)
    I2C.deinit()
    SPI.deinit()
    PWM.deinit()
    ADC.deinit()
    Timer.deinit()
    Pin.irq(handler=None)
    gc.collect()
    exit()

# Load drivers and kernel mods

try:
    for key in drivers:
        try:
            code = "\n".join(drivers[key])
            exec(code)
        except Exception:
            errorcode(0)
            BSOD()
except TypeError:
    pass

# /// System ///

while True:
    try:
        cmdlet = input("BaushaOS> ").split()

        if cmdlet[0] == "echo":
            print(" ".join(cmdlet[1:]))

        elif cmdlet[0] == "python":
            print("Welcome to Python! Use /exit to exit, or execute anything here!")
            python3 = 1
            while python3 == 1:
                code = input("py> ")
                if code == "//exit":
                    python3 = 0
                else:
                    exec(code)

        elif cmdlet[0] == "dir":
            if cmdlet[1] == "-create":
                filesystem[cmdlet[2]] = []
            elif cmdlet[1] == "-delete":
                del filesystem[cmdlet[2]]
            elif cmdlet[1] == "-show":
                print("\n".join(filesystem[cmdlet[2]]))
            
        elif cmdlet[0] == "txtedit" and 0 <= 1 < len(cmdlet):
            print("Welcome to Text Editor! Use //exit to exit, or type anything here!")
            txtedit = 1
            while txtedit == 1:
                newline = input("tx> ")
                if newline == "//exit":
                    txtedit = 0
                else:
                    filesystem[cmdlet[1]].append(newline)

        elif cmdlet[0] == "poweroff":
            exit()

        elif cmdlet[0] == "balengu":
            if cmdlet[1] == "-create":
                if cmdlet[2] == "--new":
                    apps[cmdlet[3]] = []
                elif cmdlet[2] == "--edit":
                    print("Welcome to Balengu App Maker! Use //exit to exit, or type your MicroPython here!")
                    appmake = 1
                    while appmake == 1:
                        try:
                            newline = input("ba> ")
                            if newline == "//exit":
                                appmake = 0
                            else:
                                apps[cmdlet[3]].append(newline)
                        except KeyError:
                            errorcode(2)

        elif cmdlet[0] == "aquamarine":
            if cmdlet[1] == "-create":
                if cmdlet[2] == "--new":
                    drivers[cmdlet[3]] = []
                elif cmdlet[2] == "--edit":
                    print("Welcome to Aquamarine. Use //exit to exit.")
                    print("Please know that incorrect driver code can lead to system instability or crashes, effectively bricking this OS.")
                    drivers = 1
                    while drivers == 1:
                        try:
                            newline = input("aq> ")
                            if newline == "//exit":
                                drivers = 0
                            else:
                                drivers[cmdlet[3]].append(newline)
                        except KeyError:
                            errorcode(2)
        
        elif cmdlet[0] == "apprun":
            try:
                code = "\n".join(apps[cmdlet[1]])
                exec(code)
            except Exception:
                errorcode(0)
                BSOD()
            
        elif cmdlet[0] == "random":
            if cmdlet[1] == "-sha256":
                randomnum = random.randint(0, 1000000)
                hash_object = hashlib.sha256(bytes(str(randomnum), "utf-8"))
                hexobj = hash_object.digest()
                print(hexobj)
            else:
                print(random.randint(0, int(cmdlet[1])))
            
    except IndexError:
        errorcode(1)
