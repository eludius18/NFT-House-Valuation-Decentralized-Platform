from flask import Flask, request, jsonify
from utils import load_trained_model_and_scaler, preprocess_single_house
import numpy as np

app = Flask(__name__)

# Load the trained model and scaler
model, scaler = load_trained_model_and_scaler()

@app.route("/predict", methods=["POST"])
def predict():
    data = request.json
    # Preprocess the input
    try:
        house_data_scaled = preprocess_single_house(data, scaler)
        predicted_log_price = model.predict(house_data_scaled)
        predicted_price = np.expm1(predicted_log_price)  # Reverse log transformation
        return jsonify({"price": round(float(predicted_price[0][0]), 2)})  # Convert to float
    except Exception as e:
        return jsonify({"error": str(e)}), 400

if __name__ == "__main__":
    app.run(port=5000)
