# rustbook
Rustbool is a collection of exercices provided by the book ["The Rust Programming Language"](https://doc.rust-lang.org/book/title-page.html) by Steve Klabnik and Carol Nichols.
The exercices are made using Rust 1.42.0 or later with edition="2018" in Cargo.toml of all projects to use Rust 2018 Edition idioms. 

## Print the lyrics to the Christmas carol “The Twelve Days of Christmas”, taking advantage of the repetition in the song

App prints the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

## Lyrics

On the first day of Christmas my true love sent to me
A partridge in a pear tree

On the second day of Christmas my true love sent to me
Two turtle doves
And a partridge in a pear tree

On the third day of Christmas my true love sent to me
Three French hens, 
two turtle doves,
And a partridge in a pear tree

On the fourth day of Christmas my true love sent to me
Four calling birds, 
three French hens, 
two turtle doves,
And a partridge in a pear tree

On the fifth day of Christmas my true love sent to me
Five gold rings, 
four calling birds, 
three French hens, 
two turtle doves
And a partridge in a pear tree

On the sixth day of Christmas my true love sent to me
Six geese a laying, 
five gold rings, 
four calling birds
Three French hens, 
two turtle doves
And a partridge in a pear tree

On the seventh day of Christmas my true love sent to me
Seven swans a swimming, 
six geese a laying, 
five gold rings
Four calling birds, 
three French hens, 
two turtle doves
And a partridge in a pear tree

On the eighth day of Christmas my true love sent to me
Eight maids a milking, 
seven swans a swimming, 
six geese a laying
Five gold rings, 
four calling birds, 
three French hens, 
two turtle doves
And a partridge in a pear tree

On the ninth day of Christmas my true love sent to me
Nine drummers drumming
Eight maids a milking, 
seven swans a swimming, 
six geese a laying
Five gold rings, 
four calling birds, 
three French hens, 
two turtle doves
And a partridge in a pear tree

On the tenth day of Christmas my true love sent to me
Ten pipers piping,
Nine drummers drumming,
Eight maids a milking, 
seven swans a swimming, 
six geese a laying
Five gold rings, 
four calling birds, 
three French hens, 
two turtle doves
And a partridge in a pear tree

On the eleventh day of Christmas my true love sent to me
Eleven ladies dancing,
Ten pipers piping,
Nine drummers drumming,
Eight maids a milking, 
seven swans a swimming, 
six geese a laying
Five gold rings, 
four calling birds, 
three French hens, 
two turtle doves
And a partridge in a pear tree

On the twelfth day of Christmas my true love sent to me
Twelve Lords a leaping,
Eleven ladies dancing,
Ten pipers piping,
Nine drummers drumming,
Eight maids a milking, 
seven swans a swimming, 
six geese a laying
Five gold rings, 
four calling birds, 
three French hens, 
two turtle doves
And a partridge in a pear tree, and a partridge in a pear tree

##Lyrics estructure

Lyrics has 12 stanzas. 
Each stanza begin whit the **same poem only changing the number**, which increase in one each repetition. it Then it adds one poem per stanza
and repeats all others in the last paragraph in the same order.

Apps create an array whit the first poem, that has a {} and the 12 unique poems that will repeat in each.

```
let versos = [
        "On the {} day of Christmas my true love sent to me" ,
        "And a partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine drummers drumming",
        "Ten pipers piping",
        "Eleven ladies dancing",
        "Twelve Lords a leaping",
    ];
```

Create another array with ordinal form first to twelve

```
let ordinal = ["first","second", "third", "fouth", "fifht", "sixth","seventh", "eight", "ninth","tenth","eleventh","twelfth" ];
```

App define a Loop from 1 to 12, so FOR is loop choosed, takes the first poem, add the ordinal inside {} in this poem from array "ordinal".

The first loop, it's the first stanza. It has only one poem, the first, so it need add no more poems and ends the loop

From second to twelfth, aftes prints first poems, it adds n-1 verses in reverse mode form versos array.

Last stanza repeats the last poem, that is the same as first whith a litte variation, so in code apps add an if condition.


