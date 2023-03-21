// Import required packages
require('dotenv').config()
const { readFileSync, writeFileSync } = require('jsonfile')
const { Webhook, MessageBuilder } = require("discord-webhook-node")
const rss = new (require('rss-parser'))()

// Initialize Discord webhook and asset library
const hook = new Webhook(process.env.WEBHOOK)
const library = {
    ambu: 'https://files.thijmenheuvelink.nl/api/public/dl/ginQaA4N/ambu.png',
    brand: 'https://files.thijmenheuvelink.nl/api/public/dl/ginQaA4N/brand.png',
    ongeval: 'https://files.thijmenheuvelink.nl/api/public/dl/ginQaA4N/ongeval.png',
    politie: 'https://files.thijmenheuvelink.nl/api/public/dl/ginQaA4N/politie.png',
    trauma: 'https://files.thijmenheuvelink.nl/api/public/dl/ginQaA4N/trauma.png',
}

// Check if content matches an asset keyword and return the corresponding URL
function assetMatch(content) {
    const query = content.toLowerCase()

    if (query.includes('ambu')) return library['ambu']
    if (query.includes('brand')) return library['brand']
    if (query.includes('politie')) return library['politie']
    if (query.includes('trauma')) return library['trauma']
    return library['ongeval']
}

// Poll the RSS feed for new items and send them to the Discord webhook
function poll() {
    rss.parseURL(process.env.RSS)
        .then(({ items }) => {
            let cache = readFileSync('./cache.json')

            // Check if item has already been sent and send new items to webhook
            items.reverse().forEach(item => {
                if (cache.find(({ guid }) => guid == item.guid)) return

                cache.push(item)

                console.log('Sent:', item)

                const embed = new MessageBuilder()
                    .setColor('#e36549')
                    .setURL(item.link)
                    .setTitle(item.content)
                    .setTimestamp(item.isoDate)
                    .setImage(assetMatch(item.content))

                hook.send(embed)
            })

            // Update cache with newly sent items
            writeFileSync('./cache.json', cache)
            console.log('Done, next poll in 15m.\n')
        })
}

// Initialize polling and set interval for polling every 15 minutes
poll()
setInterval(() => poll(), 15 * 60 * 1000)

// Handle uncaught exceptions
process.on('uncaughtException', err => console.error(err))
