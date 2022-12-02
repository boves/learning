import pandas as pd # URL= https://pandas.pydata.org/pandas-docs/stable/getting_started/intro_tutorials/04_plotting.html
import matplotlib.pyplot as plt
air_quality = pd.read_csv("air_quality_no2.csv", index_col=0, parse_dates=True)