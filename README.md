# List of Caboose Fractions

For an explanation on **Heegner Numbers** / **Caboose Numbers** / **Treehouse Numbers** / **Eulers Lucky Numbers** see Brady Haran and Matt Parker on Numberphile about [Caboose](https://www.youtube.com/watch?v=gM5uNcgn2NQ) and [Treehouse Numbers](https://www.youtube.com/watch?v=mw4DM1952KI&t=0s*).

(also on OEIS [here](https://oeis.org/A003173) and [here](https://oeis.org/A014556))

# What is this?

This repository contains a list of the caboose fractions for all uneven numbers up to 1mio, and a small rust script to generate them.
The only 100% caboose numbers are: **3**, **5**, **11**, **17** and **41**.

![caboose fractions to numbers up to 1mio](https://github.com/TobiasGrothmann/list_of_caboose_fractions/assets/28928394/5f584289-09b7-46c2-91c6-3125203f8fac)
Up to 60k as a scatter-plot:
![scatterplot of the numbers](https://github.com/TobiasGrothmann/list_of_caboose_fractions/assets/28928394/d116858e-c262-4b6b-a9fd-0ecaf293d76b)

A rolling average over 1000 entries leaves not much hope of finding another 100% caboose number (We also know there are none).
Going up, numbers are less and less caboose.

![all numbers as rolling average](https://github.com/TobiasGrothmann/list_of_caboose_fractions/assets/28928394/a82e7f58-c5c0-425e-abd5-b137053ff692)

# some more facts

The lowest caboosity of the uneven numbers up to 1mio. are:
| c      | caboosity |
-------- | ----------|
| 931203 | 0.015316  |
| 987525 | 0.015406  |
| 807735 | 0.015660  |
| 984873 | 0.015661  |
| 936663 | 0.015739  |
...

The highest are:

| c   | caboosity |
| --- | --------- |
| 3   | 1.0       |
| 5   | 1.0       |
| 11  | 1.0       |
| 17  | 1.0       |
| 41  | 1.0       |
| 101 | 0.686869  |
| 107 | 0.676190  |
| 7   | 0.600000  |
| 59  | 0.596491  |
| 221 | 0.589041  |
| 227 | 0.586667  |
| 47  | 0.577778  |
| 67  | 0.538462  |
| 377 | 0.509333  |
| 347 | 0.507246  |
| 161 | 0.503145  |
...

# Why?

Yeah, don't ask. I guess this counts as a hobby of some sort.

Does anyone really need this repository? - I think not.
