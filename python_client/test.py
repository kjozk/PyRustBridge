import ctypes
import os
import sys


class PyRustBridge:
    def __init__(self, lib_path=None):
        if lib_path is None:
            lib_path = self._default_library_path()
        self.lib = ctypes.CDLL(lib_path)

        # 関数定義を一括登録（C++のヘッダ相当）
        self._func_signatures = {
            "add":      (ctypes.c_int, (ctypes.c_int, ctypes.c_int)),
            "multiply": (ctypes.c_int, (ctypes.c_int, ctypes.c_int)),
        }

        # キャッシュ用辞書（何度も関数オブジェクトを作らないため）
        self._func_cache = {}

    def _default_library_path(self):
        #dir = os.path.join(os.path.dirname(__file__), "..", "rustlib", "target", "release")
        dir = os.path.join(os.path.dirname(__file__), "..", "rustlib", "target", "debug")
        if sys.platform.startswith("win"):
            return os.path.join(dir, "rustlib.dll")
        elif sys.platform.startswith("linux"):
            return os.path.join(dir, "librustlib.so")
        else:
            raise RuntimeError("Unsupported platform")

    def _bind_functions(self, func_defs):
        """辞書に基づいてライブラリ関数を登録"""
        for name, (restype, argtypes) in func_defs.items():
            func = getattr(self.lib, name)
            func.restype = restype
            func.argtypes = argtypes
            setattr(self, name, func)

    def _call_func(self, name, *args):
        # ctypes呼び出し共通処理
        if name not in self._func_cache:
            func = getattr(self.lib, name)

            # 関数シグネチャ情報がなければ AttributeError
            if name not in self._func_signatures:
                raise AttributeError(f"'{type(self).__name__}' object has no attribute '{name}'")

            # ctypes から関数オブジェクト取得
            func = getattr(self.lib, name)
            restype, argtypes = self._func_signatures[name]
            func.restype = restype
            func.argtypes = argtypes

            self._func_cache[name] = func
        else:
            func = self._func_cache[name]
        return func(*args)

    def add(self, a, b):
        return self._call_func('add', a, b)

    def multiply(self, a, b):
        return self._call_func('multiply', a, b)


if __name__ == "__main__":
    bridge = PyRustBridge()

    print("1 + 1 =", bridge.add(1, 1))
    print("2 + 3 =", bridge.add(2, 3))
    print("4 * 5 =", bridge.multiply(4, 5))
