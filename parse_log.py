#!/usr/bin/env python3

import sys

if __name__ == "__main__":
    try:
        file = sys.argv[1]
    except:
        file = "/home/jni/Downloads/manual_log_2.log"
    with open(file) as f:
        count = 0
        for line in f:
            timestamp, iface, frame = line.split(" ")
            id, frame_data = frame.split("#")
            frame_data = [int(frame_data[i:i+2], 16) for i in range(0, len(frame_data) - 4, 2)]
            params, data_bytes = frame_data[:3], frame_data[3:]

            num_data_bytes, service, pid = params

            if service != 0x41 or pid > 31 or id != "7E8":
                continue
            if pid == 12:
                count = count + 1
                print("-", data_bytes)
        print(count)
