from flask import Flask, render_template, request
import requests
from datetime import datetime, timedelta

app = Flask(__name__)

API_URL = "https://api.exchangerate.host"

@app.route('/')
def index():
    return render_template('index.html')

@app.route('/convert', methods=['GET', 'POST'])
def convert():
    result = None
    if request.method == 'POST':
        amount = request.form.get('amount', type=float, default=0.0)
        from_currency = request.form.get('from_currency', 'USD')
        to_currency = request.form.get('to_currency', 'EUR')
        try:
            resp = requests.get(f"{API_URL}/convert", params={
                'from': from_currency,
                'to': to_currency,
                'amount': amount
            })
            resp.raise_for_status()
            data = resp.json()
            result = data.get('result')
        except Exception as exc:
            result = f"Error: {exc}"
    return render_template('convert.html', result=result)

@app.route('/graph', methods=['GET', 'POST'])
def graph():
    rates = []
    labels = []
    if request.method == 'POST':
        base = request.form.get('base', 'USD')
        target = request.form.get('target', 'EUR')
        end_date = datetime.today()
        start_date = end_date - timedelta(days=6)
        try:
            resp = requests.get(f"{API_URL}/timeseries", params={
                'start_date': start_date.strftime('%Y-%m-%d'),
                'end_date': end_date.strftime('%Y-%m-%d'),
                'base': base,
                'symbols': target
            })
            resp.raise_for_status()
            data = resp.json()
            for date, rate in sorted(data.get('rates', {}).items()):
                labels.append(date)
                rates.append(rate.get(target))
        except Exception as exc:
            labels.append('Error')
            rates.append(0)
    return render_template('graph.html', labels=labels, rates=rates)

if __name__ == '__main__':
    app.run(debug=True)
