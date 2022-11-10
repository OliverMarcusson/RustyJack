function randInt(max) {
    return Math.floor((Math.random() * max) + 1)
}

class Deck {
    constructor(size) {
         this.cards = [];
        for (let k = 0; k < size; k++) {
            for (let i = 1; i < 5; i++) {
                for (let j = 2; j < 15; j++) {
                    this.cards.push(new Card(j, i, i))
                }   
            }
        }
    }

    shuffle(amount) {
        for (let j = 0; j < amount; j++) {
            for (let i = 0; i < this.cards.length; i++) {
                let rand = randInt(this.cards.length - 1)
                let card = this.cards[rand]
                this.cards.splice(rand, 1)
                this.cards.push(card)
            }
        }
    }
}

class Card {
        constructor(value, suite, color) {
            this.value = value
            this.clothed = false
            if (value == 11) {
                this.valueName = 'Knight'
                this.value = 10
                this.clothed = true
            }
            else if (value == 12) {
                this.valueName = 'Queen'
                this.value = 10
                this.clothed = true
            }
            else if (value == 13) {
                this.valueName = 'King'
                this.value = 10
                this.clothed = true
            }
            else if (value == 14) {
                this.valueName = 'Ace'
                this.value = 11
            }
            else {
                this.valueName = value
            }            
            this.suite = suite
            if (suite == 1) {
                this.suiteName = 'Spade'
            }
            else if (suite == 2) {
                this.suiteName = 'Heart'
            }
            else if (suite == 3) {
                this.suiteName = 'Diamond'    
            }
            else if (suite == 4) {
                this.suiteName = 'Clover'
            }
            else {
                this.suiteName = null
            }
            
            this.color = color
            if (color == 2 || color == 3) {
                this.colorName = 'Red'
            }
            else if (color == 1 || color == 4) {
                this.colorName = 'Black'
            }

        }
}

module.exports = {
    randInt: randInt,
    Deck: Deck,
    Card: Card
};