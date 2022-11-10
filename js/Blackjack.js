const Deck = require('./Deck')

const SHOE_SIZE = 3

class User {
    constructor(id, tokens, dealer) {
        this.id = id
        this.tokens = tokens
        this.hand = new Hand(dealer)
    }

    play(bet, deck, autoPlay) {
        if (bet > tokens) {console.log('Not enough tokens to play.'); return}
        this.tokens -= bet

        if (!autoPlay) {
            console.log('WIP')
        }
        else {
            this.hand.autoPlay(deck)
            if (win == 1) { // Win
                tokens += bet * 2
            }
            if (win == 2) { // Push 
                tokens += bet
            }
            if (win == 3) { // Surrender
                tokens += bet / 2
            }
            if (win == 4) { // Blackjack
                tokens += bet * 1.5
            }
        }
    }
}

class Hand {
    constructor(dealer) {
        this.dealer = dealer
        this.cards = []
        this.cardNames = []
        this.handValue = 0
        this.blackjack = false
    };

    draw(deck, amount) {
        if (deck.cards.length < amount) {
            console.log("Deck empty, making new one.")

        }
        else {
            for (let i = 0; i < amount; i++) {
                this.cards.push(deck.cards.pop())
            }
            this.updateCardNames()
            this.updateHandValue()
            this.checkBlackjack()
        }
    };

    updateCardNames() {
        this.cardNames = []
        for (let i = 0; i < this.cards.length; i++) {
            this.cardNames.push(`${this.cards[i].valueName} of ${this.cards[i].suiteName}s`)
        }
    }

    updateHandValue() {
        this.handValue = 0
        for (let i = 0; i < this.cards.length; i++) {
            this.handValue += this.cards[i].value
        }
        // Checks if hand contains one or more aces and adjusts the hand value accordingly. 
        if (this.handValue >= 22) {
            for (let i = 0; i < this.cards.length; i++) {
                if (this.cards[i].value == 11) { // true if card is an ace.
                    this.handValue -= 10 // makes the ace's value 1 instead of 14.
                }
            }
        }
    }

    checkBlackjack() {
        if (this.handValue != 21) {return}
        let ace = false
        let clothed = false
        for (let i = 0; i < this.cards.length; i++) {
            if (this.cards[i].clothed) {
                clothed = true
            }
            else if (this.cards[i].value == 11) {
                ace = true
            }
        }
        if (ace && clothed) {
            this.blackjack = true
        }
    }

    autoPlay(deck) {
        while (this.handValue < 16) {
            this.draw(deck, 1)
        }
    }
};

function checkWin(dealerHand, playerHand) {
    // 0: player loss, 1: player win, 2: push, 3: player surrender, 4: player blackjack.
    if (playerHand.surrender) {
        return 3
    }
    
    if (dealerHand.blackjack && playerHand.blackjack) {
        return 2
    }

    if (dealerHand.handValue == playerHand.handValue) {
        return 2
    }

    if (dealerHand.blackjack) {
        return 0
    }

    if (dealerHand.handValue > playerHand.handValue) {
        return 0
    }

    if (playerHand.handValue >= 22) {
        return 0
    }

    if (playerHand.blackjack) {
        return 4
    }

    if (dealerHand.handValue >= 22) {
        return 1
    }

    if (dealerHand.handValue < playerHand.handValue) {
        return 1
    }
}

function simulatePlay(plays) {
    let dealerWins = 0
    let playerWins = 0
    let push = 0
    let random = 0

    for (let i = 0; i < plays; i++) {
        
        let shoe = new Deck.Deck(SHOE_SIZE);
        shoe.shuffle(2);
        
        const player = new User(1, 100000, false)
        const dealer = new User(0, 0, true)

        player.hand.draw(shoe, 2);
        dealer.hand.draw(shoe, 2);
        
        player.hand.autoPlay(shoe)
        dealer.hand.autoPlay(shoe)
        
        let win = checkWin(dealer.hand, player.hand)
        if (win == 0) {
            dealerWins += 1
        }
        else if (win == 1) {
            random = Deck.randInt(500)
            if (random == 1) {console.log(`Dealer hand: ${dealer.hand.cardNames} | Value: ${dealer.hand.handValue}\nPlayer hand: ${player.hand.cardNames} | Value: ${player.hand.handValue}\n`)}
            playerWins += 1
        }
        else if (win == 4) {
            playerWins += 1
        }
        else if (win == 2) {
            push += 1
        }
    }

    console.log(`Player wins: ${playerWins}\nDealer wins: ${dealerWins}\nPush: ${push}\nPlayer winrate: ${Math.floor((playerWins + push) / (plays / 100))}% (${playerWins + push} wins)`)
}

// Main program starts here.

simulatePlay(1000000)




