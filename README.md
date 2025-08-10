

## 環境構築

### venv

```shell
cd python_client
python -m venv venv
.\venv\Scripts\Activate.ps1
pip install -r requirements.txt

```

## ビルド

### RustライブラリをReleaseビルド

```shell
cd rustlib
cargo build --release
```

### RustライブラリをDebugビルド

```shell
cd rustlib
cargo build
```

## 実行

### Pythonから呼び出す

```shell
cd ../python_client
python test.py
```

