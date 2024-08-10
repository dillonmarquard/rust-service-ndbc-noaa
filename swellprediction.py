import requests
import matplotlib.pyplot as plt
from datetime import datetime, timedelta
import numpy as np
from sklearn.gaussian_process import GaussianProcessRegressor
from sklearn.gaussian_process.kernels import RBF, ConstantKernel as C
from sklearn.preprocessing import StandardScaler

def fetch_buoy_historical_data(buoy_id, year):
    url = f"https://www.ndbc.noaa.gov/view_text_file.php?filename={buoy_id}h{year}.txt.gz&dir=data/historical/stdmet/"
    try:
        response = requests.get(url, stream=True)
        response.raise_for_status()

        times = []
        swell_heights = []
        swell_periods = []
        mean_wave_directions = []

        def round_to_nearest_half_hour(dt):
            new_minute = (dt.minute // 30) * 30
            if dt.minute % 30 >= 15:
                new_minute += 30
            if new_minute == 60:
                new_minute = 0
                dt += timedelta(hours=1)
            return dt.replace(minute=new_minute, second=0, microsecond=0)

        for line in response.iter_lines():
            line = line.decode('utf-8')
            if line.startswith('#') or line.strip() == '':
                continue
            columns = line.split()
            if len(columns) >= 13:
                date_str = f"{columns[0]} {columns[1]} {columns[2]} {columns[3]} {columns[4]}"
                date_time = datetime.strptime(date_str, '%Y %m %d %H %M')
                date_time = round_to_nearest_half_hour(date_time)
                swell_height = float(columns[8])
                if swell_height <= 40:
                    if date_time.minute != 30:
                        times.append(date_time)
                        swell_heights.append(swell_height)
                        swell_periods.append(float(columns[9]))
                        mean_wave_directions.append(float(columns[11]))

        return times, swell_heights, swell_periods, mean_wave_directions

    except requests.exceptions.HTTPError as http_err:
        print(f"HTTP error occurred: {http_err}")
        raise
    except Exception as err:
        print(f"Error occurred while fetching data: {err}")
        raise

def fetch_buoy_realtime_data(buoy_id):
    try:
        url = f"https://www.ndbc.noaa.gov/data/realtime2/{buoy_id}.spec"
        response = requests.get(url)
        response.raise_for_status()
        lines = response.text.splitlines()
        headers = lines[0].split()
        data_lines = lines[2:]

        times = []
        swell_heights = []
        swell_periods = []
        mean_wave_directions = []

        def round_to_nearest_half_hour(dt):
            new_minute = (dt.minute // 30) * 30
            if dt.minute % 30 >= 15:
                new_minute += 30
            if new_minute == 60:
                new_minute = 0
                dt += timedelta(hours=1)
            return dt.replace(minute=new_minute, second=0, microsecond=0)

        for line in data_lines:
            columns = line.split()
            if len(columns) >= 8 and columns[8] != 'MM' and columns[9] != 'MM' and columns[5] != 'MM':
                date_str = f"{columns[0]} {columns[1]} {columns[2]} {columns[3]} {columns[4]}"
                date_time = datetime.strptime(date_str, '%Y %m %d %H %M')
                date_time = round_to_nearest_half_hour(date_time)
                swell_height = float(columns[5])
                if swell_height <= 40:
                    if date_time.minute != 30:
                        times.append(date_time)
                        swell_heights.append(swell_height)
                        swell_periods.append(float(columns[7]))
                        mean_wave_directions.append(float(columns[14]))

        times.reverse()
        swell_heights.reverse()
        swell_periods.reverse()
        mean_wave_directions.reverse()

        return times, swell_heights, swell_periods, mean_wave_directions

    except requests.exceptions.HTTPError as http_err:
        print(f"HTTP error occurred: {http_err}")
        raise
    except Exception as err:
        print(f"Error occurred while fetching data: {err}")
        raise

# Buoy data fetching and prediction
buoy_ids = ["46224", "46258"]  # List of buoy IDs
historical_years = ["2022", "2023"]  # List of historical years

for buoy_id in buoy_ids:
    historical_times = []
    historical_swell_heights = []
    historical_swell_periods = []
    historical_mean_wave_directions = []

    # Fetch historical data for each year
    for year in historical_years:
        year_times, year_swell_heights, year_swell_periods, year_mean_wave_directions = fetch_buoy_historical_data(buoy_id, year)
        historical_times.extend(year_times)
        historical_swell_heights.extend(year_swell_heights)
        historical_swell_periods.extend(year_swell_periods)
        historical_mean_wave_directions.extend(year_mean_wave_directions)

    # Fetch real-time data
    realtime_times, realtime_swell_heights, realtime_swell_periods, realtime_mean_wave_directions = fetch_buoy_realtime_data(buoy_id)

    # Combine historical and real-time data
    combined_times = historical_times + realtime_times
    combined_swell_heights = historical_swell_heights + realtime_swell_heights

    # Convert times to ordinal for modeling
    combined_times_ordinal = np.array([dt.toordinal() for dt in combined_times]).reshape(-1, 1)
    combined_swell_heights = np.array(combined_swell_heights).reshape(-1, 1)

    # Scale the data
    scaler_time = StandardScaler()
    scaler_height = StandardScaler()
    combined_times_scaled = scaler_time.fit_transform(combined_times_ordinal)
    combined_swell_heights_scaled = scaler_height.fit_transform(combined_swell_heights)

    # Simplified kernel: Constant * RBF
    # kernel = C(1.0, (1, 10)) * RBF(length_scale=1.0, length_scale_bounds=(1, 10))

    # Create the Gaussian Process model
    # gp = GaussianProcessRegressor(kernel=kernel, n_restarts_optimizer=10)

    # Train the model
    # gp.fit(combined_times_scaled, combined_swell_heights_scaled.ravel())

    # Generate future dates for prediction (e.g., next 30 days)
    future_days = 30
    last_date = combined_times[-1]
    future_times = np.array([last_date + timedelta(days=i) for i in range(1, future_days + 1)])
    future_times_ordinal = np.array([dt.toordinal() for dt in future_times]).reshape(-1, 1)
    future_times_scaled = scaler_time.transform(future_times_ordinal)

    # Predict future swell heights
    # predicted_swell_heights_scaled, sigma = gp.predict(future_times_scaled, return_std=True)
    # predicted_swell_heights = scaler_height.inverse_transform(predicted_swell_heights_scaled)

    # Plot the results
    plt.figure(figsize=(10, 6))
    plt.plot(combined_times, combined_swell_heights, 'b.', markersize=10, label='Observed Swell Heights')
    plt.plot(future_times, predicted_swell_heights, 'r-', label='Predicted Swell Heights')
    plt.fill_between(future_times, predicted_swell_heights - sigma, predicted_swell_heights + sigma, color='r', alpha=0.2)
    plt.xlabel('Date')
    plt.ylabel('Swell Height (m)')
    plt.title(f'Swell Height Prediction for Buoy {buoy_id}')
    plt.legend()
    plt.grid(True)
    plt.show()