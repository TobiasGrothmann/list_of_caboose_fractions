import pandas as pd
import matplotlib

with open("../caboose_fractions.csv", "r") as f:
    # create pandas df
    df = pd.read_csv(f)

# plot the data
df.plot(x='c', y='fraction', ylabel="caboose fraction", title='Caboose Fractions', legend=False)
matplotlib.pyplot.show()

# plot a rolling average
window_size = 1000
df['rolling_avg'] = df['fraction'].rolling(window=window_size).mean()
df.plot(x='c', y='rolling_avg', ylabel="caboose fraction, rolling avg", title='Caboose Fractions (rolling avg, window-size 1000)', legend=False)
matplotlib.pyplot.show()
