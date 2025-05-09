from flask import Flask, jsonify
from sklearn.ensemble import IsolationForest
import pandas as pd

app = Flask(__name__)

@app.route("/risk")
def calculate_risk():
    data = pd.DataFrame({
        "amount": [10, 200, 50000, 15, 300, 1000000],
        "frequency": [5, 1, 2, 7, 1, 0]
    })
    model = IsolationForest(contamination=0.2)
    data['risk'] = model.fit_predict(data)
    return jsonify(data.to_dict(orient='records'))

if __name__ == "__main__":
    app.run(host='0.0.0.0', port=5000)