set style data filledcurves x1
set key outside right
unset border
set title 'Amount of cards in each area in camicia'
plot 'data2.data' using 0:1 lw 3 title 'First player',\
     'data2.data' using 0:2 lw 3 title 'Second player',\
     'data2.data' using 0:3 lw 3 title 'Table'
