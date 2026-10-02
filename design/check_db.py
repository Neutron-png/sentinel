import sqlite3

conn = sqlite3.connect(r"D:\Sentainal\sentinel.db")
rows = conn.execute(
    "SELECT method, url, status_code, response_size, duration_ms,"
    " request_headers != '', response_headers != ''"
    " FROM history_entries ORDER BY timestamp DESC LIMIT 6"
).fetchall()
for r in rows:
    print(r)
conn.close()
