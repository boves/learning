import pandas as pd # URL= https://pandas.pydata.org/pandas-docs/stable/getting_started/intro_tutorials/04_plotting.html
import matplotlib.pyplot as plt; import os; os.chdir(r"./pandas/")
air_quality = pd.read_csv("data/air_quality_no2.csv", index_col=0, parse_dates=True) # os.chdir(r"./pandas/"") if this doesn't work
air_quality.head()
air_quality.plot()
plt.show()
air_quality["station_paris"].plot()
plt.show()
air_quality.plot.scatter(x="station_london", y="station_paris", alpha=0.5)
plt.show() 
[
    method_name
    for method_name in dir(air_quality.plot)
    if not method_name.startswith("_")
]
air_quality.plot.box()
plt.show()
axs = air_quality.plot.area(figsize=(12, 4), subplots=True)
plot.show()










