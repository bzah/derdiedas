# DerDieDas

## Status

Not even v0: don't use it.

## Goal
Typical `derdiedas` apps are guessing games to learn the German articles for the nominative case.
Here the goal is similar, we want to provide a CLI application where based on open data, we display
a German word on the nominative case and expect the user to input a the proper article.

In addition, we want to provide a colored coded helper which would be displayed on top of the word to guess,
to indicate the probable article based on word start or ending.
For example, most word ending in `-or` correspond to the article `der`. 
Thus, for words where this is true such as `Autor` the ending would be highlighted with the pre-configured `der` color.
See the section below for the guess tips and their corresponding article.
This would be helpful to learn both the article and the logic to recognize them.
Afterwards, we could also add an option to start the game with only exceptions as they must be memorized.

## Roadmap v0
- Make a cli accepting as argument the number of word to guess.
- Define the model necessary to store words, their article, the most probable article and if its an exception.
- Select the word to guess
- Display the selected word with the possible highlight or a `!` before it if it's an exception.
- Prompt the 3 articles and let the user select them via a keyboard shortcut.
- Display if it is correct or and error, then continue with the next guess.
- Repeat until the number of guess is reached.
- Display total score

## Ideas for v1
- [corpus] Collect the source of truth corpus from open data (e.g. wikitionnary)
  It might be interesting to have multiple corpus for different level of learners, like the 5000 most used words.
- [corpus] Compute statistics for each guess tip (e.g. ending `-or`) and display this in the guessing game.
  The most useful statistic would probably be the % of words in the corpus having the guessed article and for whom the guess tip is true.
- [guess-tips] Instead of my home-made guess tips, find a reliable source in the linguistic literature and update our guess tips accordingly.
- [refactoring] Split the engine from the prompt game to be able to compile it in webassembly and port the game to the web/mobile in the futur.
- [game] Display successes and errors at the end of the game.
- [game] Keep a history of past games to only display unknown words.
- [corpus|game] Store and display a translation in English.

## Masculine (der)

### Always masculine
Masculine people : der Vater
Days of the week : der Montag
Months: der Januar
Seasons: Der Sommer
Precipitation: der Regen

#### 16 Word Endings
| Ending   | Example                                                |
| -------- | ------------------------------------------------------ |
| us       | Der Eukalyptus, Der Bus                                |
| ismus    | Der Kapitalismus                                       |
| or       | Der Autor, Der Motor                                   |
| ist      | Der Tourist                                            |
| en       | Der Garten, Der Ofen                                   |
| er       | Der Baker                                              |
| ich      | Der Teppich                                            |
| eich     | Der Bereich (the area)                                 |
| ig       | Der Essig                                              |
| eig      | Der Zweig                                              |
| ant      | Der Dilletant, Der Elefant                             |
| anz      | Der Tanz (only monosyllabic, otherwise Feminine)       |
| ast      | Der Palast                                             |
| s        | Der Ausweis                                            |
| ling     | Der Schmetterling (the butterfly)                      |
| m        | Der Film                                               |
| s        | Des Ausweis                                            |


### Usually masculines:
Times of the day: Der Morgen
Alcohol:               Der Wein
Names of rivers:  Der Rhein
Names of mountains: Der Mount Everest

## Feminine (die)

### Always Feminine
Feminine people: Die Mutter
Cardinal numbers: Die Eins

#### 7 Endings
| Ending   | Example                                                |
| -------- | ------------------------------------------------------ |
| heit    | Die Freiheit                                            |
| keit    | Die Fröhlichkeit (the happiness)                        |
| ik      | Die Musik                                               |
| schaft  | Die Gesellshaft (la société)                            |
| ur      | Die Faktur, die Kultur (usually similar to french -ur)  |
| ität    | Die Identiät, Die Universität                           |
| ung     | Die Zeitung, Die Leisung                                |

### Usually feminine
Names of plants/trees: Die Rose

#### 7 Endings
| Ending   | Example                                                |
| -------- | ------------------------------------------------------ |
| e        | Die Lampe                                              |
| ei       | Die Backerei, Die Partei                               |
| ie       | Die Magie                                              |
| enz      | Die Frequenz                                           |
| anz      | Die Toleranz (not monosyllabic, these are maculines)   |
| ios      | Die Diskussion                                         |
| in       | Die Studentin (female persons)                         |


## Neutral (das)

### Always neutral 
Diminutives: Das Mäuschen (the little mouse)
Color: Das rot
Nominalized vergs: Das Lesen (the reading)
Nominalized adjectives: Das Gute (the good)


### Usually neutral
Loan word from English: Das Baby
Names of metals: Das Gold
Names of chemical el.: Das Aluminium
Fraction numbers: Das Drittel (the third)


#### Endings
| Ending     | Example                                                   |
| --------   | --------------------------------------------------------- |
| chen       | Das Mäuschen                                              |
| ment       | Das Parlament                                             | 
| nis        | Das Ergenis (the result), Das Zeugnis (the certificat)    |
| o          | Das Auto                                                  |
| um         | Das Zentrum                                               |
| tum        | Das Herzogtum (the duchy)                                 |
