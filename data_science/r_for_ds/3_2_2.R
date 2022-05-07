library(tidyverse)

ggplot(data = mpg) +
    geom_point(mapping = aes(x = displ, y = hwy, color = class))

ggsave("plot.pdf")










# check for Rplots.pdf and remove it
file.exists("Rplots.pdf")
file.remove("Rplots.pdf")
