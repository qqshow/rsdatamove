import ctypes

client = ctypes.CDLL('./libclient.so')
client.initialize()

data = b"Hello, Server!"
response = client.send_data(data, len(data))
print(ctypes.string_at(response))
client.free_data(response)