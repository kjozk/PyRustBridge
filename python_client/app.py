from flask import Flask, jsonify, request, render_template
import threading
import time
import random

from PyRustBridge import PyRustBridge

app = Flask(__name__)
rust_lib = PyRustBridge()

job_last_access = {}
job_lock = threading.Lock()

def generate_job_id() -> int:
    # 適当なユニークID生成 (例えば乱数)
    return random.randint(1, 1_000_000_000)


@app.route("/")
def index():
    return render_template("index.html")

@app.route("/start_job", methods=["POST"])
def start_job():
    job_id = generate_job_id()
    with job_lock:
        job_last_access[job_id] = time.time()
    rust_lib.start_job(job_id)
    return jsonify({"job_id": job_id})

@app.route("/job_status/<int:job_id>", methods=["GET"])
def job_status(job_id):
    with job_lock:
        job_last_access[job_id] = time.time()
    progress = rust_lib.get_progress(job_id)
    return jsonify({"progress": progress})

@app.route("/cancel_job/<int:job_id>", methods=["POST"])
def cancel_job(job_id):
    with job_lock:
        job_last_access.pop(job_id, None)
    rust_lib.cancel_job(job_id)
    return jsonify({"result": "canceled"})

def cleanup_jobs():
    while True:
        now = time.time()
        timeout = 60  # 60秒アクセスがなければキャンセル
        with job_lock:
            to_cancel = [jid for jid, last in job_last_access.items() if now - last > timeout]
        for jid in to_cancel:
            rust_lib.cancel_job(jid)
            with job_lock:
                job_last_access.pop(jid, None)
        time.sleep(10)

if __name__ == "__main__":
    threading.Thread(target=cleanup_jobs, daemon=True).start()
    app.run(debug=True)
