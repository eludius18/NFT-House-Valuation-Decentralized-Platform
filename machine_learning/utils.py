from tensorflow.keras.models import load_model
import joblib
import pandas as pd

def load_trained_model_and_scaler():
    model = load_model('house_price_model.keras')
    scaler = joblib.load('scaler.save')
    return model, scaler

def preprocess_single_house(house_data, scaler):
    house_df = pd.DataFrame([house_data])
    missing_features = set(scaler.feature_names_in_) - set(house_df.columns)
    for feature in missing_features:
        house_df[feature] = 0  # Add default value for missing features
    house_df = house_df[scaler.feature_names_in_]  # Ensure column order matches training
    return scaler.transform(house_df)
