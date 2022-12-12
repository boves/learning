import pandas as pd # URL = https://pandas.pydata.org/pandas-docs/stable/getting_started/intro_tutorials/06_calculate_statistics.html
titanic = pd.read_csv("https://raw.githubusercontent.com/pandas-dev/pandas/main/doc/data/titanic.csv")
titanic.head()
titanic["Age"].mean()
titanic[["Age", "Fare"]].median()
titanic[["Age", "Fare"]].describe()
titanic.agg(
    {
        "Age": ["min", "max", "median", "skew"],
        "Fare": ["min", "max", "median", "mean"],
    }
)
titanic.groupby("Sex").mean(numeric_only=True)  # line 9
titanic[["Sex", "Age"]].groupby("Sex").mean()   # line 10
titanic.groupby(["Sex", "Pclass"])["Fare"].mean() # line 11
titanic["Pclass"].value_counts() # line 12
titanic.groupby("Pclass")["Pclass"].count() # line 13