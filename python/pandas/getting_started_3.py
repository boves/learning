import pandas as pd # URL= https://pandas.pydata.org/pandas-docs/stable/getting_started/intro_tutorials/05_add_columns.html
air_quality = pd.read_csv("data/air_quality_no2.csv", index_col=0, parse_dates=True) # os.chdir(r"./pandas/") if this doesn't work
air_quality.head()
air_quality["london_mg_per_cubic"] = air_quality["station_london"] * 1.882
air_quality.head()
air_quality["ratio_paris_antwerp"] = (
    air_quality["station_paris"] / air_quality["station_antwerp"]
)
air_quality.head()
air_quality_renamed = air_quality_renamed.rename(columns=str.lower)

air_quality_renamed.head()