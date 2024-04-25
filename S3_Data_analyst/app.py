from flask import Flask, render_template
import plotly.graph_objects as go
from plotly.subplots import make_subplots
import pymysql, json
import pandas as pd

app = Flask(__name__)

with open('config.json', 'r') as file:
    config = json.load(file)
# MySQL database connection details
db_host = config['database']['host']
db_name = config['database']['dbname']
db_username = config['database']['username']
db_password = config['database']['password']

@app.route('/', methods=['GET', 'POST'])
def home():
    conn = pymysql.connect(host=db_host, user=db_username, passwd=db_password, db=db_name)
    cursor = conn.cursor()
    cursor.execute("SELECT timestamp, value, price FROM HP_USAGE_138372")
    data = cursor.fetchall()
    times = [row[0] for row in data]
    values = [row[1] for row in data]
    prices = [row[2] for row in data]

    cursor.close()
    conn.close()

    fig = make_subplots(rows=1, cols=1, specs=[[{'secondary_y': True}]])
    fig.add_trace(go.Scatter(x=times, y=values, mode='lines', marker=dict(color='green'), name='Watt Hour'), row=1, col=1, secondary_y=False)
    fig.add_trace(go.Scatter(x=times, y=prices, mode='lines', marker=dict(color='orange'), name='Price'), row=1, col=1, secondary_y=True)
    fig.update_yaxes(title_text='Watt Hour', secondary_y=False)
    fig.update_yaxes(title_text='Price', secondary_y=True)
    fig.update_layout(title='Pulse Data Line Plot', xaxis_title='Time')


    last_7_days = pd.Timestamp.now() - pd.DateOffset(days=7)
    fig.update_xaxes(range=[last_7_days, pd.Timestamp.now()])
    chart = fig.to_html(full_html=False)

    return render_template('index.html', chart=chart)


if __name__ == '__main__':
    from waitress import serve
    #serve(app, host="0.0.0.0", port=80)
    app.run(host='0.0.0.0', port=5001, debug=True)