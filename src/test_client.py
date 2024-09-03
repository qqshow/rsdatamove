'''
Author: x-star
Date: 2024-09-02 18:13:23
LastEditors: x-star
LastEditTime: 2024-09-03 14:32:38
FilePath: /rsdatamove/src/test_client.py
Description: 

Copyright (c) 2024 by Epic, All Rights Reserved. 
'''
import ctypes
import os

# 获取当前脚本的目录
current_dir = os.path.dirname(os.path.abspath(__file__))
# 构建动态库的路径
lib_path = os.path.join(current_dir, '..', 'target', 'release', 'libclient.dylib')

client = ctypes.CDLL(lib_path)

# 定义函数参数类型和返回类型
client.initialize.argtypes = []
client.initialize.restype = None

client.send_data.argtypes = [ctypes.c_char_p, ctypes.c_size_t]
client.send_data.restype = ctypes.c_void_p

client.free_data.argtypes = [ctypes.c_void_p]
client.free_data.restype = None

client.initialize()

data = b"Hello, Server! I'm python client"
response = client.send_data(data, len(data))
print(ctypes.string_at(response).decode('utf-8'))
client.free_data(response)