from PyRustBridge import PyRustBridge

if __name__ == "__main__":
    bridge = PyRustBridge()

    print("1 + 1 =", bridge.add(1, 1))
    print("2 + 3 =", bridge.add(2, 3))
    print("4 * 5 =", bridge.multiply(4, 5))
